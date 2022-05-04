use envoy_sdk::extension::{entrypoint, Module, Result};

use crate::filter::WasabiHttpFilterFactory;

mod filter;

entrypoint! { initialize }

fn initialize() -> Result<Module> {
    Module::new().add_http_filter(|_instance_id| WasabiHttpFilterFactory::default())
}
