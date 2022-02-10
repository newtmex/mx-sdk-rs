use std::{collections::HashMap, path::PathBuf, rc::Rc, str::FromStr};

use elrond_wasm::{
    contract_base::{CallableContract, ContractBase},
    elrond_codec::{TopDecode, TopEncode},
    types::{Address, EsdtLocalRole, H256},
};

use crate::{
    rust_biguint,
    testing_framework::raw_converter::bytes_to_hex,
    tx_execution::interpret_panic_as_tx_result,
    tx_mock::{TxCache, TxContext, TxContextStack, TxInput, TxInputESDT, TxResult},
    world_mock::{is_smart_contract_address, AccountData, AccountEsdt, EsdtInstanceMetadata},
    BlockchainMock, DebugApi,
};

use super::{
    tx_mandos::{ScCallMandos, TxExpectMandos},
    AddressFactory, MandosGenerator, ScQueryMandos,
};

pub struct ContractObjWrapper<
    CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
    ContractObjBuilder: 'static + Copy + Fn() -> CB,
> {
    pub(crate) address: Address,
    pub(crate) obj_builder: ContractObjBuilder,
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
    ContractObjBuilder: 'static + Copy + Fn() -> CB,
{
    pub(crate) fn new(address: Address, obj_builder: ContractObjBuilder) -> Self {
        ContractObjWrapper {
            address,
            obj_builder,
        }
    }

    pub fn address_ref(&self) -> &Address {
        &self.address
    }
}

pub struct BlockchainStateWrapper {
    address_factory: AddressFactory,
    rc_b_mock: Rc<BlockchainMock>,
    address_to_code_path: HashMap<Address, Vec<u8>>,
    mandos_generator: MandosGenerator,
    workspace_path: PathBuf,
}

impl BlockchainStateWrapper {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push(PathBuf::from_str("mandos/").unwrap());

        BlockchainStateWrapper {
            address_factory: AddressFactory::new(),
            rc_b_mock: Rc::new(BlockchainMock::new()),
            address_to_code_path: HashMap::new(),
            mandos_generator: MandosGenerator::new(),
            workspace_path: current_dir,
        }
    }

    pub fn get_mut_state(&mut self) -> &mut Rc<BlockchainMock> {
        &mut self.rc_b_mock
    }

    pub fn write_mandos_output(self, file_name: &str) {
        let mut full_path = self.workspace_path;
        full_path.push(file_name);

        self.mandos_generator
            .write_mandos_output(full_path.to_str().unwrap());
    }

    pub fn check_egld_balance(&self, address: &Address, expected_balance: &num_bigint::BigUint) {
        let actual_balance = match &self.rc_b_mock.accounts.get(address) {
            Some(acc) => acc.egld_balance.clone(),
            None => rust_biguint!(0),
        };

        assert!(
            expected_balance == &actual_balance,
            "EGLD balance mismatch for address {}\n Expected: {}\n Have: {}\n",
            address_to_hex(address),
            expected_balance,
            actual_balance
        );
    }

    pub fn check_esdt_balance(
        &self,
        address: &Address,
        token_id: &[u8],
        expected_balance: &num_bigint::BigUint,
    ) {
        let actual_balance = match &self.rc_b_mock.accounts.get(address) {
            Some(acc) => acc.esdt.get_esdt_balance(token_id, 0),
            None => rust_biguint!(0),
        };

        assert!(
            expected_balance == &actual_balance,
            "ESDT balance mismatch for address {}\n Token: {}\n Expected: {}\n Have: {}\n",
            address_to_hex(address),
            String::from_utf8(token_id.to_vec()).unwrap(),
            expected_balance,
            actual_balance
        );
    }

    pub fn check_nft_balance<T>(
        &self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        expected_balance: &num_bigint::BigUint,
        expected_attributes: &T,
    ) where
        T: TopEncode + TopDecode + PartialEq + core::fmt::Debug,
    {
        let actual_attributes_serialized = match &self.rc_b_mock.accounts.get(address) {
            Some(acc) => {
                let esdt_data = acc.esdt.get_by_identifier_or_default(token_id);
                let opt_instance = esdt_data.instances.get_by_nonce(nonce);

                match opt_instance {
                    Some(instance) => {
                        assert!(
                            expected_balance == &instance.balance,
                            "ESDT NFT balance mismatch for address {}\n Token: {}, nonce: {}\n Expected: {}\n Have: {}\n",
                            address_to_hex(address),
                            String::from_utf8(token_id.to_vec()).unwrap(),
                            nonce,
                            expected_balance,
                            instance.balance
                        );

                        instance.metadata.attributes.clone()
                    },
                    None => Vec::new(),
                }
            },
            None => Vec::new(),
        };

        let actual_attributes = T::top_decode(actual_attributes_serialized).unwrap();
        assert!(
            expected_attributes == &actual_attributes,
            "ESDT NFT attributes mismatch for address {}\n Token: {}, nonce: {}\n Expected: {:?}\n Have: {:?}\n",
            address_to_hex(address),
            String::from_utf8(token_id.to_vec()).unwrap(),
            nonce,
            expected_attributes,
            actual_attributes,
        );
    }
}

impl BlockchainStateWrapper {
    pub fn create_user_account(&mut self, egld_balance: &num_bigint::BigUint) -> Address {
        let address = self.address_factory.new_address();
        self.create_account_raw(&address, egld_balance, None, None, None);

        address
    }

    pub fn create_user_account_fixed_address(
        &mut self,
        address: &Address,
        egld_balance: &num_bigint::BigUint,
    ) {
        self.create_account_raw(address, egld_balance, None, None, None);
    }

    pub fn create_sc_account<CB, ContractObjBuilder>(
        &mut self,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
        obj_builder: ContractObjBuilder,
        contract_wasm_path: &str,
    ) -> ContractObjWrapper<CB, ContractObjBuilder>
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
    {
        let address = self.address_factory.new_sc_address();
        self.create_sc_account_fixed_address(
            &address,
            egld_balance,
            owner,
            obj_builder,
            contract_wasm_path,
        )
    }

    pub fn create_sc_account_fixed_address<CB, ContractObjBuilder>(
        &mut self,
        address: &Address,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
        obj_builder: ContractObjBuilder,
        contract_wasm_path: &str,
    ) -> ContractObjWrapper<CB, ContractObjBuilder>
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
    {
        if !is_smart_contract_address(address) {
            panic!("Invalid SC Address: {:?}", address_to_hex(address))
        }

        let mut wasm_full_path = std::env::current_dir().unwrap();
        wasm_full_path.push(PathBuf::from_str(contract_wasm_path).unwrap());

        let path_diff =
            pathdiff::diff_paths(wasm_full_path.clone(), self.workspace_path.clone()).unwrap();
        let path_str = path_diff.to_str().unwrap();

        let wasm_full_path_as_expr = "file:".to_owned() + wasm_full_path.to_str().unwrap();
        let contract_bytes = mandos::value_interpreter::interpret_string(
            &wasm_full_path_as_expr,
            &mandos::interpret_trait::InterpreterContext::new(std::path::PathBuf::new()),
        );

        let wasm_relative_path_expr = "file:".to_owned() + path_str;
        let was_relative_path_expr_bytes = wasm_relative_path_expr.as_bytes().to_vec();

        self.address_to_code_path
            .insert(address.clone(), was_relative_path_expr_bytes.clone());

        self.create_account_raw(
            address,
            egld_balance,
            owner,
            Some(contract_bytes),
            Some(was_relative_path_expr_bytes),
        );

        if !self.rc_b_mock.contains_contract(&wasm_full_path_as_expr) {
            let contract_obj = create_contract_obj_box(obj_builder);

            let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
            b_mock_ref.register_contract_obj(&wasm_full_path_as_expr, contract_obj);
        }

        ContractObjWrapper::new(address.clone(), obj_builder)
    }

    pub fn create_account_raw(
        &mut self,
        address: &Address,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
        sc_identifier: Option<Vec<u8>>,
        sc_mandos_path_expr: Option<Vec<u8>>,
    ) {
        if self.rc_b_mock.account_exists(address) {
            panic!("Address already used: {:?}", address_to_hex(address));
        }

        let acc_data = AccountData {
            address: address.clone(),
            nonce: 0,
            egld_balance: egld_balance.clone(),
            esdt: AccountEsdt::default(),
            storage: HashMap::new(),
            username: Vec::new(),
            contract_path: sc_identifier,
            contract_owner: owner.cloned(),
        };
        self.mandos_generator
            .set_account(&acc_data, sc_mandos_path_expr);

        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.add_account(acc_data);
    }

    pub fn set_egld_balance(&mut self, address: &Address, balance: &num_bigint::BigUint) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        match b_mock_ref.accounts.get_mut(address) {
            Some(acc) => {
                acc.egld_balance = balance.clone();

                self.add_mandos_set_account(address);
            },

            None => panic!(
                "set_egld_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_esdt_balance(
        &mut self,
        address: &Address,
        token_id: &[u8],
        balance: &num_bigint::BigUint,
    ) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        match b_mock_ref.accounts.get_mut(address) {
            Some(acc) => {
                acc.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    0,
                    balance,
                    EsdtInstanceMetadata::default(),
                );

                self.add_mandos_set_account(address);
            },
            None => panic!(
                "set_esdt_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_nft_balance<T: TopEncode>(
        &mut self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        balance: &num_bigint::BigUint,
        attributes: &T,
    ) {
        self.set_nft_balance_all_properties(
            address, token_id, nonce, balance, attributes, 0, None, None, None, None,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_nft_balance_all_properties<T: TopEncode>(
        &mut self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        balance: &num_bigint::BigUint,
        attributes: &T,
        royalties: u64,
        creator: Option<&Address>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uri: Option<&[u8]>,
    ) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        match b_mock_ref.accounts.get_mut(address) {
            Some(acc) => {
                acc.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    nonce,
                    balance,
                    EsdtInstanceMetadata {
                        creator: creator.cloned(),
                        attributes: serialize_attributes(attributes),
                        royalties,
                        name: name.unwrap_or_default().to_vec(),
                        hash: hash.map(|h| h.to_vec()),
                        uri: uri.map(|u| u.to_vec()),
                    },
                );

                self.add_mandos_set_account(address);
            },
            None => panic!(
                "set_nft_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_esdt_local_roles(
        &mut self,
        address: &Address,
        token_id: &[u8],
        roles: &[EsdtLocalRole],
    ) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        match b_mock_ref.accounts.get_mut(address) {
            Some(acc) => {
                let mut roles_raw = Vec::new();
                for role in roles {
                    roles_raw.push(role.as_role_name().to_vec());
                }
                acc.esdt.set_roles(token_id.to_vec(), roles_raw);

                self.add_mandos_set_account(address);
            },
            None => panic!(
                "set_esdt_local_roles: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_block_epoch(&mut self, block_epoch: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.current_block_info.block_epoch = block_epoch;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_block_nonce(&mut self, block_nonce: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.current_block_info.block_nonce = block_nonce;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.current_block_info.block_random_seed = block_random_seed;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_block_round(&mut self, block_round: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.current_block_info.block_round = block_round;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_block_timestamp(&mut self, block_timestamp: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.current_block_info.block_timestamp = block_timestamp;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_epoch(&mut self, block_epoch: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.previous_block_info.block_epoch = block_epoch;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_nonce(&mut self, block_nonce: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.previous_block_info.block_nonce = block_nonce;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.previous_block_info.block_random_seed = block_random_seed;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_round(&mut self, block_round: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.previous_block_info.block_round = block_round;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_timestamp(&mut self, block_timestamp: u64) {
        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        b_mock_ref.previous_block_info.block_timestamp = block_timestamp;

        self.mandos_generator.set_block_info(
            &self.rc_b_mock.current_block_info,
            &self.rc_b_mock.previous_block_info,
        );
    }

    pub fn add_mandos_sc_call(
        &mut self,
        sc_call: ScCallMandos,
        opt_expect: Option<TxExpectMandos>,
    ) {
        self.mandos_generator
            .create_tx(&sc_call, opt_expect.as_ref());
    }

    pub fn add_mandos_sc_query(
        &mut self,
        sc_query: ScQueryMandos,
        opt_expect: Option<TxExpectMandos>,
    ) {
        self.mandos_generator
            .create_query(&sc_query, opt_expect.as_ref());
    }

    pub fn add_mandos_set_account(&mut self, address: &Address) {
        if let Some(acc) = self.rc_b_mock.accounts.get(address) {
            let opt_contract_path = self.address_to_code_path.get(address);
            self.mandos_generator
                .set_account(acc, opt_contract_path.cloned());
        }
    }

    pub fn add_mandos_check_account(&mut self, address: &Address) {
        if let Some(acc) = self.rc_b_mock.accounts.get(address) {
            self.mandos_generator.check_account(acc);
        }
    }
}

impl BlockchainStateWrapper {
    pub fn execute_tx<CB, ContractObjBuilder, TxFn>(
        &mut self,
        caller: &Address,
        sc_wrapper: &ContractObjWrapper<CB, ContractObjBuilder>,
        egld_payment: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> TxResult
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
        TxFn: FnOnce(CB),
    {
        self.execute_tx_any(caller, sc_wrapper, egld_payment, Vec::new(), tx_fn)
    }

    pub fn execute_esdt_transfer<CB, ContractObjBuilder, TxFn>(
        &mut self,
        caller: &Address,
        sc_wrapper: &ContractObjWrapper<CB, ContractObjBuilder>,
        token_id: &[u8],
        esdt_nonce: u64,
        esdt_amount: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> TxResult
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
        TxFn: FnOnce(CB),
    {
        let esdt_transfer = vec![TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce: esdt_nonce,
            value: esdt_amount.clone(),
        }];
        self.execute_tx_any(caller, sc_wrapper, &rust_biguint!(0), esdt_transfer, tx_fn)
    }

    pub fn execute_esdt_multi_transfer<CB, ContractObjBuilder, TxFn: FnOnce(CB)>(
        &mut self,
        caller: &Address,
        sc_wrapper: &ContractObjWrapper<CB, ContractObjBuilder>,
        esdt_transfers: &[TxInputESDT],
        tx_fn: TxFn,
    ) -> TxResult
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
    {
        self.execute_tx_any(
            caller,
            sc_wrapper,
            &rust_biguint!(0),
            esdt_transfers.to_vec(),
            tx_fn,
        )
    }

    pub fn execute_query<CB, ContractObjBuilder, TxFn: FnOnce(CB)>(
        &mut self,
        sc_wrapper: &ContractObjWrapper<CB, ContractObjBuilder>,
        query_fn: TxFn,
    ) -> TxResult
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
    {
        self.execute_tx(
            sc_wrapper.address_ref(),
            sc_wrapper,
            &rust_biguint!(0),
            query_fn,
        )
    }

    // deduplicates code for execution
    fn execute_tx_any<CB, ContractObjBuilder, TxFn: FnOnce(CB)>(
        &mut self,
        caller: &Address,
        sc_wrapper: &ContractObjWrapper<CB, ContractObjBuilder>,
        egld_payment: &num_bigint::BigUint,
        esdt_payments: Vec<TxInputESDT>,
        tx_fn: TxFn,
    ) -> TxResult
    where
        CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
        ContractObjBuilder: 'static + Copy + Fn() -> CB,
    {
        let sc_address = sc_wrapper.address_ref();
        let tx_cache = TxCache::new(self.rc_b_mock.clone());
        let rust_zero = rust_biguint!(0);

        if egld_payment > &rust_zero {
            tx_cache.subtract_egld_balance(caller, egld_payment);
            tx_cache.increase_egld_balance(sc_address, egld_payment);
        }

        for esdt in &esdt_payments {
            if esdt.value > rust_zero {
                let metadata = tx_cache.subtract_esdt_balance(
                    caller,
                    &esdt.token_identifier,
                    esdt.nonce,
                    &esdt.value,
                );
                tx_cache.increase_esdt_balance(
                    sc_address,
                    &esdt.token_identifier,
                    esdt.nonce,
                    &esdt.value,
                    metadata,
                );
            }
        }

        let tx_input = build_tx_input(caller, sc_address, egld_payment, esdt_payments);
        let tx_context_rc = Rc::new(TxContext::new(tx_input, tx_cache));
        TxContextStack::static_push(tx_context_rc);

        let sc = (sc_wrapper.obj_builder)();
        let exec_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tx_fn(sc)));

        let api_after_exec = Rc::try_unwrap(TxContextStack::static_pop()).unwrap();
        let updates = api_after_exec.into_blockchain_updates();

        let b_mock_ref = Rc::get_mut(&mut self.rc_b_mock).unwrap();
        match exec_result {
            Ok(()) => {
                // do not commit changes for SC Query (caller == SC in that case)
                if caller != sc_wrapper.address_ref() {
                    updates.apply(b_mock_ref);
                }

                TxResult::empty()
            },
            Err(panic_any) => interpret_panic_as_tx_result(panic_any),
        }
    }

    pub fn execute_in_managed_environment<T, Func: FnOnce() -> T>(&self, f: Func) -> T {
        let _ = DebugApi::dummy();
        let result = f();
        let _ = TxContextStack::static_pop();

        result
    }
}

impl BlockchainStateWrapper {
    pub fn get_egld_balance(&self, address: &Address) -> num_bigint::BigUint {
        match self.rc_b_mock.accounts.get(address) {
            Some(acc) => acc.egld_balance.clone(),
            None => panic!(
                "get_egld_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn get_esdt_balance(
        &self,
        address: &Address,
        token_id: &[u8],
        token_nonce: u64,
    ) -> num_bigint::BigUint {
        match self.rc_b_mock.accounts.get(address) {
            Some(acc) => acc.esdt.get_esdt_balance(token_id, token_nonce),
            None => panic!(
                "get_esdt_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn get_nft_attributes<T: TopDecode>(
        &self,
        address: &Address,
        token_id: &[u8],
        token_nonce: u64,
    ) -> Option<T> {
        match self.rc_b_mock.accounts.get(address) {
            Some(acc) => match acc.esdt.get_by_identifier(token_id) {
                Some(esdt_data) => esdt_data
                    .instances
                    .get_by_nonce(token_nonce)
                    .map(|inst| T::top_decode(inst.metadata.attributes.clone()).unwrap()),
                None => None,
            },
            None => panic!(
                "get_nft_attributes: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn dump_state(&self) {
        for addr in self.rc_b_mock.accounts.keys() {
            self.dump_state_for_account_hex_attributes(addr);
            println!();
        }
    }

    #[inline]
    /// Prints the state for the account, with any token attributes as hex
    pub fn dump_state_for_account_hex_attributes(&self, address: &Address) {
        self.dump_state_for_account::<Vec<u8>>(address)
    }

    /// Prints the state for the account, with token attributes decoded as the provided type, if possible
    pub fn dump_state_for_account<AttributesType: 'static + TopDecode + core::fmt::Debug>(
        &self,
        address: &Address,
    ) {
        let account = match self.rc_b_mock.accounts.get(address) {
            Some(acc) => acc,
            None => panic!(
                "dump_state_for_account: Account {:?} does not exist",
                address_to_hex(address)
            ),
        };

        println!("State for account: {:?}", address_to_hex(address));
        println!("EGLD: {}", account.egld_balance);

        if !account.esdt.is_empty() {
            println!("ESDT Tokens:");
        }
        for (token_id, acc_esdt) in account.esdt.iter() {
            let token_id_str = String::from_utf8(token_id.to_vec()).unwrap();
            println!("  Token: {}", token_id_str);

            for (token_nonce, instance) in acc_esdt.instances.get_instances() {
                if std::any::TypeId::of::<AttributesType>() == std::any::TypeId::of::<Vec<u8>>() {
                    print_token_balance_raw(
                        *token_nonce,
                        &instance.balance,
                        &instance.metadata.attributes,
                    );
                } else {
                    match AttributesType::top_decode(&instance.metadata.attributes[..]) {
                        core::result::Result::Ok(attr) => {
                            print_token_balance_specialized(*token_nonce, &instance.balance, &attr)
                        },
                        core::result::Result::Err(_) => print_token_balance_raw(
                            *token_nonce,
                            &instance.balance,
                            &instance.metadata.attributes,
                        ),
                    }
                }
            }
        }

        if !account.storage.is_empty() {
            println!();
            println!("Storage: ");
        }
        for (key, value) in &account.storage {
            let key_str = match String::from_utf8(key.to_vec()) {
                core::result::Result::Ok(s) => s,
                core::result::Result::Err(_) => bytes_to_hex(key),
            };
            let value_str = bytes_to_hex(value);

            println!("{}: {}", key_str, value_str);
        }
    }
}

fn build_tx_input(
    caller: &Address,
    dest: &Address,
    egld_value: &num_bigint::BigUint,
    esdt_values: Vec<TxInputESDT>,
) -> TxInput {
    TxInput {
        from: caller.clone(),
        to: dest.clone(),
        egld_value: egld_value.clone(),
        esdt_values,
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: u64::MAX,
        gas_price: 0,
        tx_hash: H256::zero(),
    }
}

fn address_to_hex(address: &Address) -> String {
    hex::encode(address.as_bytes())
}

fn serialize_attributes<T: TopEncode>(attributes: &T) -> Vec<u8> {
    let mut serialized_attributes = Vec::new();
    if let Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
        panic!("Failed to encode attributes: {:?}", err)
    }

    serialized_attributes
}

fn print_token_balance_raw(
    token_nonce: u64,
    token_balance: &num_bigint::BigUint,
    attributes: &[u8],
) {
    println!(
        "      Nonce {}, balance: {}, attributes: {}",
        token_nonce,
        token_balance,
        bytes_to_hex(attributes)
    );
}

fn print_token_balance_specialized<T: core::fmt::Debug>(
    token_nonce: u64,
    token_balance: &num_bigint::BigUint,
    attributes: &T,
) {
    println!(
        "      Nonce {}, balance: {}, attributes: {:?}",
        token_nonce, token_balance, attributes
    );
}

fn create_contract_obj_box<CB, ContractObjBuilder>(
    func: ContractObjBuilder,
) -> Box<dyn CallableContract>
where
    CB: ContractBase<Api = DebugApi> + CallableContract + 'static,
    ContractObjBuilder: 'static + Fn() -> CB,
{
    let c_base = func();
    Box::new(c_base)
}
