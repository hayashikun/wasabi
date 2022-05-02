use std::cmp::min;

use image::{imageops, RgbImage};
use tract_onnx::prelude::*;

struct CenterFace {
    width: usize,
    height: usize,
    model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl CenterFace {
    pub fn new(width: usize, height: usize) -> TractResult<CenterFace> {
        let ws = width / 32;
        let hs = height / 32;
        let model = onnx().model_for_path("resource/centerface.onnx")?
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 3, height as i32, width as i32]))?
            .with_output_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 1, 8 * hs, 8 * ws]))?
            .with_output_fact(1, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 2, 8 * hs, 8 * ws]))?
            .with_output_fact(2, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 2, 8 * hs, 8 * ws]))?
            .with_output_fact(3, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 10, 8 * hs, 8 * ws]))?
            .into_optimized()?
            .into_runnable()?;

        Ok(CenterFace { width, height, model })
    }

    pub fn detect(&self, mut input: RgbImage) -> TractResult<TVec<Arc<Tensor>>> {
        let org_width = input.width();
        let org_height = input.height();
        let crop_size = min(org_width, org_height);
        let cropped = imageops::crop(
            &mut input,
            (org_width - crop_size) / 2, (org_height - crop_size) / 2,
            crop_size, crop_size,
        ).to_image();
        let image = imageops::resize(
            &cropped,
            self.width as u32, self.height as u32,
            imageops::FilterType::Triangle,
        );
        let image: Tensor = tract_ndarray::Array4::from_shape_fn(
            (1, 3, self.height, self.width),
            |(_, c, y, x)| {
                let mean = [0.485, 0.456, 0.406][c];
                let std = [0.229, 0.224, 0.225][c];
                (image[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
            },
        ).into();

        self.model.run(tvec!(image))
    }
}

#[cfg(test)]
mod tests {
    use crate::center_face::CenterFace;

    #[test]
    fn model_works() {
        let cf = CenterFace::new(640, 480);
        let image = image::open("resource/cabinet.jpg").unwrap().to_rgb8();

        let result = cf.unwrap().detect(image);
        println!("{:?}", result);
    }
}
