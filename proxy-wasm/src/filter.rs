use std::io::Cursor;

use envoy_sdk::extension::{ConfigStatus, ExtensionFactory, factory, HttpFilter, InstanceId, Result};
use envoy_sdk::extension::filter::http::{FilterDataStatus, RequestBodyOps};
use envoy_sdk::host::{ByteString, log};
use jpeg_decoder::Decoder;
use tract_onnx::prelude::Tensor;
use tract_onnx::prelude::tract_ndarray::Array4;

use wasabi::center_face::{CenterFace, Face};

pub struct WasabiHttpFilter {
    cf: CenterFace,
    req_body: Vec<u8>,
}

impl WasabiHttpFilter {
    pub fn new() -> Self {
        WasabiHttpFilter {
            cf: CenterFace::new(640, 480).unwrap(),
            req_body: vec![],
        }
    }
}


impl Default for WasabiHttpFilter {
    fn default() -> Self {
        WasabiHttpFilter::new()
    }
}


impl HttpFilter for WasabiHttpFilter {
    fn on_request_body(
        &mut self,
        data_size: usize,
        end_of_stream: bool,
        ops: &dyn RequestBodyOps,
    ) -> Result<FilterDataStatus> {
        let bs = ops.request_data(0, data_size)?;
        self.req_body.append(&mut bs.to_vec());

        if !end_of_stream {
            return Ok(FilterDataStatus::Continue);
        }

        let bytes = self.req_body.as_slice();

        let mut decoder = Decoder::new(Cursor::new(bytes));

        let pixels = decoder.decode()?;
        log::info!("pixels: {:?}", pixels.len());

        let width = 640;
        let height = 480;

        let image: Tensor = Array4::from_shape_fn(
            (1, 3, height, width),
            |(_, c, y, x)| {
                pixels[(y * width + x) * 4 + c] as f32
            },
        ).into();
        let faces: Vec<Face> = self.cf.detect(image).unwrap();
        log::info!("{:?}", faces);

        self.req_body = vec![];
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
