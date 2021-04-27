use super::method_gen;
use crate::model::Method;

pub fn generate_module_getter_impl(
	m: &Method,
	_impl_path: &proc_macro2::TokenTree,
) -> proc_macro2::TokenStream {
	let msig = method_gen::generate_sig(&m);
	if !m.method_args.is_empty() {
		panic!("module getter cannot have arguments");
	}

	quote! {
		#msig {
			elrond_wasm::api::new_contract_impl(self.api.clone())
		}
	}
}
