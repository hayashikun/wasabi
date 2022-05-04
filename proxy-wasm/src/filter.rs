use envoy_sdk::extension::{ConfigStatus, ExtensionFactory, factory, HttpFilter, InstanceId, Result};
use envoy_sdk::extension::filter::http;
use envoy_sdk::host::{
    ByteString, HttpClientRequestHandle, HttpClientResponseOps, log,
};

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
    fn on_request_headers(
        &mut self,
        num_headers: usize,
        end_of_stream: bool,
        _filter_ops: &dyn http::RequestHeadersOps,
    ) -> Result<http::FilterHeadersStatus> {
        log::info!("on_request_headers, {:?}, {:?}", num_headers, end_of_stream);
        Ok(http::FilterHeadersStatus::Continue)
    }

    fn on_response_headers(
        &mut self,
        num_headers: usize,
        end_of_stream: bool,
        _filter_ops: &dyn http::ResponseHeadersOps,
    ) -> Result<http::FilterHeadersStatus> {
        log::info!("on_response_headers, {:?}, {:?}", num_headers, end_of_stream);
        Ok(http::FilterHeadersStatus::Continue)
    }

    fn on_response_body(
        &mut self,
        data_size: usize,
        end_of_stream: bool,
        _ops: &dyn http::ResponseBodyOps,
    ) -> Result<http::FilterDataStatus> {
        log::info!("on_response_body, {:?}, {:?}", data_size, end_of_stream);
        Ok(http::FilterDataStatus::Continue)
    }

    fn on_exchange_complete(
        &mut self,
        _ops: &dyn http::ExchangeCompleteOps,
    ) -> Result<()> {
        log::info!("on_exchange_complete");
        Ok(())
    }

    fn on_http_call_response(
        &mut self,
        request: HttpClientRequestHandle,
        num_headers: usize,
        body_size: usize,
        num_trailers: usize,
        _filter_ops: &dyn http::Ops,
        _http_client_ops: &dyn HttpClientResponseOps,
    ) -> Result<()> {
        log::info!("on_http_call_response, {:?}, {:?}, {:?}, {:?}", request, num_headers, body_size, num_trailers);
        Ok(())
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
