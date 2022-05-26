use std::collections::HashMap;
use console_error_panic_hook;
use tract_onnx::prelude::Tensor;
use tract_onnx::prelude::tract_ndarray::Array4;
use wasm_bindgen::prelude::*;

use wasabi::center_face::{CenterFace, Face};

#[wasm_bindgen]
pub struct App {
    width: u32,
    height: u32,
    cf: CenterFace,
}

#[wasm_bindgen]
impl App {
    pub fn new(width: u32, height: u32) -> App {
        console_error_panic_hook::set_once();

        App {
            width,
            height,
            cf: CenterFace::new(width, height).unwrap(),
        }
    }

    pub fn detect(&self, array: Vec<u8>) -> String {
        let w = self.width as usize;
        let h = self.height as usize;
        let image: Tensor = Array4::from_shape_fn(
            (1, 3, h, w),
            |(_, c, y, x)| {
                array[(y * w + x) * 4 + c] as f32
            },
        ).into();
        let faces: Vec<Face> = self.cf.detect(image).unwrap();
        let mut result = HashMap::new();
        result.insert("faces", faces);

        serde_json::to_string(&result).unwrap_or("{}".to_string())
    }
}
