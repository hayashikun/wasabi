use envoy_sdk::extension::{ConfigStatus, ExtensionFactory, factory, HttpFilter, InstanceId, Result};
use envoy_sdk::extension::filter::http::{FilterDataStatus, RequestBodyOps};
use envoy_sdk::host::{ByteString, log};

pub struct WasabiHttpFilter {}

impl WasabiHttpFilter {
    pub fn new() -> Self {
        WasabiHttpFilter {}
    }
}


impl Default for WasabiHttpFilter {
    fn default() -> Self {
        WasabiHttpFilter {}
    }
}


impl HttpFilter for WasabiHttpFilter {
    fn on_request_body(
        &mut self,
        data_size: usize,
        end_of_stream: bool,
        ops: &dyn RequestBodyOps,
    ) -> Result<FilterDataStatus> {
        if !end_of_stream {
            return Ok(FilterDataStatus::Continue);
        }

        let _bytes = ops.request_data(0, data_size)?;
        log::info!("read bytes");

        Ok(FilterDataStatus::Continue)
    }
}


pub struct WasabiHttpFilterFactory {}

impl WasabiHttpFilterFactory {
    pub fn new() -> Result<Self> {
        Ok(WasabiHttpFilterFactory {})
    }

    pub fn default() -> Result<Self> {
        Self::new()
    }
}

impl ExtensionFactory for WasabiHttpFilterFactory {
    type Extension = WasabiHttpFilter;

    fn name() -> &'static str {
        "wasabi.http_filter"
    }

    fn on_configure(
        &mut self,
        _config: ByteString,
        _ops: &dyn factory::ConfigureOps,
    ) -> Result<ConfigStatus> {
        Ok(ConfigStatus::Accepted)
    }

    fn new_extension(&mut self, _instance_id: InstanceId) -> Result<Self::Extension> {
        Ok(WasabiHttpFilter::new())
    }
}
