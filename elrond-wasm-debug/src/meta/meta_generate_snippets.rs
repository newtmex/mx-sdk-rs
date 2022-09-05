use std::{
    fs::{self, File},
    io::Write,
};

use elrond_wasm::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use super::meta_config::MetaConfig;

impl MetaConfig {
    // TODO: Handle overwrite flag
    pub fn generate_rust_snippets(&self) {
        if let Some(contract) = &self.main_contract {
            let crate_name = contract.output_base_name.clone().replace("-", "_");
            let wasm_output_file_path_expr = format!("\"file:../output/{}.wasm\"", &crate_name);
            let file =
                create_snippets_crate_and_get_lib_file(&self.snippets_dir, &crate_name, true);
            write_snippets_to_file(
                file,
                &contract.abi,
                &crate_name,
                &wasm_output_file_path_expr,
            );
        }
    }
}

#[must_use]
fn create_snippets_crate_and_get_lib_file(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_snippets_folder(snippets_folder_path);
    create_snippets_gitignore(snippets_folder_path, overwrite);
    create_snippets_cargo_toml(snippets_folder_path, contract_crate_name, overwrite);
    create_src_folder(snippets_folder_path);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

fn create_snippets_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let _ = fs::create_dir(snippets_folder_path);
}

fn create_snippets_gitignore(snippets_folder_path: &str, overwrite: bool) {
    let gitignore_path = format!("{}/.gitignore", snippets_folder_path);
    let mut file = if overwrite {
        File::create(&gitignore_path).unwrap()
    } else {
        match File::options().create_new(true).open(&gitignore_path) {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        "# Pem files are used for interactions, but shouldn't be committed
*.pem"
    )
    .unwrap();
}

fn create_snippets_cargo_toml(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) {
    let cargo_toml_path = format!("{}/Cargo.toml", snippets_folder_path);
    let mut file = if overwrite {
        File::create(&cargo_toml_path).unwrap()
    } else {
        match File::options().create_new(true).open(&cargo_toml_path) {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        "[package]
name = \"rust-interact\"
version = \"0.0.0\"
authors = [\"you\"]
edition = \"2018\"
publish = false

[[bin]]
name = \"rust-interact\"
path = \"src/lib.rs\"

[dependencies.{}]
path = \"..\"

[dependencies.elrond-interact-snippets]
version = \"0.1.0\"
",
        contract_crate_name
    )
    .unwrap();
}

fn create_src_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = format!("{}/src", snippets_folder_path);
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
fn create_and_get_lib_file(snippets_folder_path: &str, overwrite: bool) -> File {
    let lib_path = format!("{}/src/lib.rs", snippets_folder_path);
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("lib.rs file already exists, overwrite option was not provided"),
        }
    }
}

fn write_snippets_to_file(
    mut file: File,
    abi: &ContractAbi,
    contract_crate_name: &str,
    wasm_output_file_path_expr: &str,
) {
    write_snippet_imports(&mut file, contract_crate_name);
    write_snippet_constants(&mut file);
    write_contract_type_alias(&mut file, contract_crate_name);
    write_snippet_main_function(&mut file, abi);
    write_state_struct_declaration(&mut file);
    write_state_struct_impl(&mut file, abi, wasm_output_file_path_expr);
}

fn write_snippet_imports(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "#[allow(non_snake_case)]

use {}::ProxyTrait as _;
use elrond_interact_snippets::{{
    elrond_wasm::{{
        elrond_codec::multi_types::{{MultiValueVec, TopDecode}},
        storage::mappers::SingleValue,
        types::{{Address, CodeMetadata}},
    }},
    elrond_wasm_debug::{{
        bech32, mandos::interpret_trait::InterpreterContext, mandos_system::model::*, ContractInfo,
        DebugApi,
    }},
    env_logger,
    erdrs::interactors::wallet::Wallet,
    tokio, Interactor,
}};
use std::{{
    env::Args,
    io::{{Read, Write}},
}};",
        contract_crate_name
    )
    .unwrap();

    write_newline(file);
}

fn write_snippet_constants(file: &mut File) {
    writeln!(file, "const GATEWAY: &str = elrond_interact_snippets::erdrs::blockchain::rpc::DEVNET_GATEWAY;
const PEM: &str = \"alice.pem\";
const SC_ADDRESS: &str = \"\";

const SYSTEM_SC_BECH32: &str = \"erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u\";
const DEFAULT_ADDRESS_EXPR: &str = \"0x0000000000000000000000000000000000000000000000000000000000000000\";
const DEFAULT_GAS_LIMIT: u64 = 100_000_000;
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;").unwrap();

    write_newline(file);
}

fn write_contract_type_alias(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "type ContractType = ContractInfo<{}::Proxy<DebugApi>>;",
        contract_crate_name
    )
    .unwrap();

    write_newline(file);
}

fn write_snippet_main_function(file: &mut File, abi: &ContractAbi) {
    writeln!(
        file,
        "#[tokio::main]
async fn main() {{
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect(\"at least one argument required\");
    let mut state = State::new().await;
    match cmd.as_str() {{"
    )
    .unwrap();

    // all contracts have a deploy and upgrade snippet
    writeln!(
        file,
        "        \"deploy\" => state.deploy().await,
        \"upgrade\" => state.upgrade().await,"
    )
    .unwrap();

    for endpoint in &abi.endpoints {
        writeln!(
            file,
            "        \"{}\" => state.{}().await,",
            endpoint.name, endpoint.name
        )
        .unwrap();
    }

    // general case of "command not found" + close curly brackets
    writeln!(
        file,
        "        _ => panic!(\"unknown command: {{}}\", &cmd),
    }}
}}"
    )
    .unwrap();

    write_newline(file);
}

fn write_state_struct_declaration(file: &mut File) {
    writeln!(
        file,
        "struct State {{
    interactor: Interactor,
    wallet_address: Address,
    contract: ContractType,
}}"
    )
    .unwrap();

    write_newline(file);
}

fn write_state_struct_impl(file: &mut File, abi: &ContractAbi, wasm_output_file_path_expr: &str) {
    writeln!(
        file,
        "impl State {{
    async fn new() -> Self {{
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == \"\" {{
            DEFAULT_ADDRESS_EXPR.to_string()
        }} else {{
            \"bech32:\".to_string() + SC_ADDRESS
        }};
        let contract = ContractType::new(sc_addr_expr);

        State {{
            interactor,
            wallet_address,
            contract,
        }}
    }}\n"
    )
    .unwrap();

    write_deploy_method_impl(file, &abi.constructors[0], wasm_output_file_path_expr);

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, endpoint_abi);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_deploy_method_impl(
    file: &mut File,
    init_abi: &EndpointAbi,
    wasm_output_file_path_expr: &str,
) {
    write_method_declaration(file, "deploy");
    write_endpoint_args_declaration(file, &init_abi.inputs);

    writeln!(
        file,
        "        let contract_deploy = ContractDeploy::<DebugApi, MultiValueVec<Vec<u8>>>::new();"
    )
    .unwrap();

    // the variables were previously declared in `write_endpoint_args_declaration`
    for input in &init_abi.inputs {
        writeln!(
            file,
            "        contract_deploy.push_endpoint_arg(&{});",
            input.arg_name
        )
        .unwrap();
    }

    write_newline(file);

    writeln!(
        file,
        "        let b_call: ScDeployStep = contract_deploy
            .into_blockchain_call()
            .from(&self.wallet_address)
            .code_metadata(CodeMetadata::all())
            .contract_code(
                {},
                &InterpreterContext::default(),
            )
            .gas_limit(DEFAULT_GAS_LIMIT)
            .expect(TxExpect::ok())
            .into();",
        wasm_output_file_path_expr,
    )
    .unwrap();

    writeln!(
        file,
        "        let results: InteractorResult<MultiValueVec<Vec<u8>>> =
            self.interactor.sc_deploy(b_call).await;
    
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!(\"new address: {{}}\", new_address_bech32);"
    )
    .unwrap();

    write_call_results_print(file, &init_abi.outputs);

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_endpoint_impl(file: &mut File, endpoint_abi: &EndpointAbi) {
    write_method_declaration(file, endpoint_abi.name);
    write_payments_declaration(file, endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
    write_contract_call(file, endpoint_abi);
    write_call_results_print(file, &endpoint_abi.outputs);

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    writeln!(file, "    async fn {}(&mut self) {{", endpoint_name).unwrap();
}

fn write_payments_declaration(file: &mut File, accepted_tokens: &[&str]) {
    if accepted_tokens.is_empty() {
        return;
    }

    // only handle EGLD and "any" case, as they're the most common
    let first_accepted = accepted_tokens[0];
    if first_accepted == "EGLD" {
        writeln!(file, "        let egld_amount = 0u64;").unwrap();
    } else {
        writeln!(
            file,
            "        let token_id = b\"\";
        let token_nonce = 0u64;
        let token_amount = 0u64;"
        )
        .unwrap();
    }

    write_newline(file);
}

/// TODO: Use an actual Rust type instead of invalid `type_name`
fn write_endpoint_args_declaration(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs {
        writeln!(
            file,
            "        let {}: {} = Default::default();",
            input.arg_name, input.type_name
        )
        .unwrap();
    }

    write_newline(file);
}

fn write_contract_call(file: &mut File, endpoint_abi: &EndpointAbi) {
    writeln!(
        file,
        "        let sc_addr = self.contract.address.clone().into_option().unwrap();
        let mut contract_call =
            ContractCall::<DebugApi, MultiValueVec<Vec<u8>>>::new(sc_addr, \"{}\");",
        endpoint_abi.name
    )
    .unwrap();

    // the variables were previously declared in `write_endpoint_args_declaration`
    for input in &endpoint_abi.inputs {
        writeln!(
            file,
            "        contract_call.push_endpoint_arg(&{});",
            input.arg_name
        )
        .unwrap();
    }

    write_newline(file);

    let payment_snippet = if endpoint_abi.payable_in_tokens.is_empty() {
        ""
    } else if endpoint_abi.payable_in_tokens[0] == "EGLD" {
        "            .egld_value(egld_amount)\n"
    } else {
        "            .esdt_transfer(token_id, token_nonce, token_amount)\n"
    };

    let call_snippet = "        let b_call: ScCallStep = contract_call
            .into_blockchain_call()
            .from(&self.wallet_address)\n"
        .to_string()
        + payment_snippet
        + "            .gas_limit(DEFAULT_GAS_LIMIT)
            .into();";

    writeln!(file, "{}", &call_snippet).unwrap();

    if endpoint_abi.outputs.is_empty() {
        writeln!(file, "        self.interactor.sc_call(b_call).await;").unwrap();
    } else {
        writeln!(
            file,
            "        let results: InteractorResult<MultiValueVec<Vec<u8>>> =
            self.interactor.sc_call_get_result(b_call).await;"
        )
        .unwrap();

        write_newline(file);
    }
}

fn write_call_results_print(file: &mut File, outputs: &[OutputAbi]) {
    if outputs.is_empty() {
        return;
    }

    writeln!(file, "        let raw_result_values = results.value().0;").unwrap();

    for (i, output) in outputs.iter().enumerate() {
        let output_name = format!("out{}", i);

        writeln!(
            file,
            "        let {} = {}::top_decode(raw_result_values[{}]).unwrap();",
            output_name, output.type_name, i
        )
        .unwrap();
    }

    write_newline(file);

    for (i, _output) in outputs.iter().enumerate() {
        let output_name = format!("out{}", i);

        writeln!(
            file,
            "        println!(\"{}: {{}}\", {})",
            output_name, output_name
        )
        .unwrap();
    }

    write_newline(file);
}

fn write_newline(file: &mut File) {
    file.write(b"\n").unwrap();
}
