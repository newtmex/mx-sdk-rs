mod meta_abi;
mod meta_cargo_toml;
mod meta_config;
mod meta_generate_snippets;
mod meta_main;
mod meta_validate_abi;
mod meta_wasm_build;
mod meta_wasm_clean;
mod meta_wasm_crates;
pub mod multi_contract;

pub use meta_main::perform;
