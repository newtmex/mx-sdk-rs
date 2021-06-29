elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::common_methods::CallbackProxy;
use crate::utils::structs::Token;
use crate::{common_methods, events, storage};

#[elrond_wasm_derive::module]
pub trait SemiFungibleTokenModule:
	storage::StorageModule + events::EventsModule + common_methods::CommonMethods
{
	#[payable("EGLD")]
	#[endpoint(sftIssue)]
	fn issue(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<Self::SendApi> {
		let caller = self.blockchain().get_caller();

		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_semi_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				SemiFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().nft_issue_callback(caller))
	}

	#[endpoint(sftAddQuantity)]
	fn add_quantity(&self, identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) {
		self.send()
			.esdt_nft_add_quantity(&identifier, nonce, &amount);

		self.bonding_curve(&Token { identifier, nonce })
			.update(|bonding_curve| {
				bonding_curve.arguments.available_supply += &amount;
				bonding_curve.arguments.balance += amount;
			});
	}
}
