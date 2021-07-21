elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait DeployContractModule {
	#[proxy]
	fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

	#[endpoint]
	fn deploy_contract(&self, code: BoxedBytes) -> SCResult<Address> {
		let deployed_contract_address = self.deploy_vault(code).ok_or("Deploy failed")?;

		Ok(deployed_contract_address)
	}

	#[endpoint]
	fn deploy_two_contracts(&self, code: BoxedBytes) -> SCResult<MultiResult2<Address, Address>> {
		let first_deployed_contract_address = self
			.deploy_vault(code.clone())
			.ok_or("First deploy failed")?;

		let second_deployed_contract_address =
			self.deploy_vault(code).ok_or("Second deploy failed")?;

		Ok((
			first_deployed_contract_address,
			second_deployed_contract_address,
		)
			.into())
	}

	#[endpoint]
	fn deploy_vault(&self, code: BoxedBytes) -> Option<Address> {
		self.vault_proxy()
			.init()
			.with_code(code, CodeMetadata::DEFAULT)
			.execute()
	}
}
