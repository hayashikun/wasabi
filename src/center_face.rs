use std::cmp::{max, min};

use image::{imageops, RgbImage};
use tract_onnx::prelude::*;
use tract_onnx::prelude::tract_itertools::Itertools;
use tract_onnx::prelude::tract_ndarray::{Array4, ArrayViewD};

struct CenterFace {
    width: u32,
    height: u32,
    model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

#[derive(Debug, Clone)]
struct Face {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    score: f32,
    landmarks: Vec<(u32, u32)>,
}

impl CenterFace {
    pub fn new(width: u32, height: u32) -> TractResult<CenterFace> {
        let ws = (width / 32) as i32;
        let hs = (height / 32) as i32;
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

    pub fn detect(&self, input: &RgbImage) -> TractResult<Vec<Face>> {
        let org_width = input.width();
        let org_height = input.height();

        let result = self.run(input)?;

        let heatmap = result.get(0).unwrap().to_array_view::<f32>()?;
        let scale = result.get(1).unwrap().to_array_view::<f32>()?;
        let offset = result.get(2).unwrap().to_array_view::<f32>()?;
        let landmark = result.get(3).unwrap().to_array_view::<f32>()?;

        let mut faces = self.decode(heatmap, scale, offset, landmark)?;

        for i in 0..faces.len() {
            faces[i].x1 = faces[i].x1 * org_width / self.width;
            faces[i].x2 = faces[i].x2 * org_width / self.width;
            faces[i].y1 = faces[i].y1 * org_height / self.height;
            faces[i].y2 = faces[i].y2 * org_height / self.height;

            for j in 0..5 {
                faces[i].landmarks[j].0 = faces[i].landmarks[j].0 * org_width / self.width;
                faces[i].landmarks[j].1 = faces[i].landmarks[j].1 * org_height / self.height;
            }
        }

        Ok(faces)
    }

    fn run(&self, input: &RgbImage) -> TractResult<TVec<Arc<Tensor>>> {
        let image = imageops::resize(
            input, self.width, self.height, imageops::FilterType::Triangle,
        );
        let image: Tensor = Array4::from_shape_fn(
            (1, 3, self.height as usize, self.width as usize),
            |(_, c, y, x)| {
                image[(x as _, y as _)][c] as f32
            },
        ).into();

        self.model.run(tvec!(image))
    }

    fn decode(
        &self,
        heatmap: ArrayViewD<f32>,
        scale: ArrayViewD<f32>,
        offset: ArrayViewD<f32>,
        landmark: ArrayViewD<f32>,
    ) -> TractResult<Vec<Face>> {
        let threshold = 0.5;

        let ny = heatmap.shape()[2];
        let nx = heatmap.shape()[3];

        let mut faces = Vec::new();

        for i in 0..ny {
            for j in 0..nx {
                let score = heatmap[[0, 0, i, j]];
                if score <= threshold {
                    continue;
                }

                let s0 = scale[[0, 0, i, j]].exp() * 4.0;
                let s1 = scale[[0, 1, i, j]].exp() * 4.0;
                let o0 = offset[[0, 0, i, j]] + 0.5;
                let o1 = offset[[0, 1, i, j]] + 0.5;

                let x1 = max(0, ((j as f32 + o1) * 4.0 - s1 / 2.0) as u32);
                let y1 = max(0, ((i as f32 + o0) * 4.0 - s0 / 2.0) as u32);

                let x1 = min(x1, self.width);
                let y1 = min(y1, self.height);

                let x2 = min(x1 + s1 as u32, self.width);
                let y2 = min(y1 + s1 as u32, self.height);

                let mut landmarks = Vec::new();
                for k in 0..5 {
                    landmarks.push((
                        x1 + (landmark[[0, 2 * k + 1, i, j]] * s1) as u32,
                        y1 + (landmark[[0, 2 * k, i, j]] * s0) as u32
                    ));
                }

                faces.push(Face { x1, x2, y1, y2, score, landmarks });
            }
        }

        Ok(Self::nms(faces))
    }

    fn nms(faces: Vec<Face>) -> Vec<Face> {
        let threshold = 0.3;

        let faces: Vec<Face> = faces.into_iter()
            .sorted_by(|a, b| b.score.partial_cmp(&a.score).unwrap())
            .collect();
        let n = faces.len();
        let mut merged = vec![false; n];

        let mut output = Vec::new();

        for i in 0..n {
            if merged[i] { continue; }

            output.push(faces[i].clone());

            let area0 = (faces[i].x2 - faces[i].x1 + 1) * (faces[i].y2 - faces[i].y1 + 1);

            for j in i + 1..n {
                if merged[j] { continue; }

                let in_x0 = if faces[i].x1 > faces[j].x1 { faces[i].x1 } else { faces[j].x1 };
                let in_y0 = if faces[i].y1 > faces[j].y1 { faces[i].y1 } else { faces[j].y1 };
                let in_x1 = if faces[i].x2 < faces[j].x2 { faces[i].x2 } else { faces[j].x2 };
                let in_y1 = if faces[i].y2 < faces[j].y2 { faces[i].y2 } else { faces[j].y2 };

                if in_x1 <= in_x0 - 1 || in_y1 <= in_y0 - 1 { continue; }
                let in_area = (in_y1 - in_y0 + 1) * (in_x1 - in_x0 + 1);
                let area1 = (faces[j].y2 - faces[j].y1 + 1) * (faces[j].x2 - faces[j].x1 + 1);

                let score = in_area as f32 / (area0 + area1 - in_area) as f32;
                if score > threshold {
                    merged[j] = true;
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::center_face::CenterFace;

    #[test]
    fn model_works() {
        let cf = CenterFace::new(32 * 15, 32 * 20).unwrap();
        let mut image = image::open("resource/cabinet.jpg").unwrap().to_rgb8();

        let faces = cf.detect(&image).unwrap();

        for f in faces {
            for x in f.x1..f.x2 {
                for y in f.y1..f.y2 {
                    let pixel = image.get_pixel(x, y);
                    let pixel: Rgb<u8> = Rgb(
                        [255 - pixel.0[0], 255 - pixel.0[1], 255 - pixel.0[2]]
                    );
                    image.put_pixel(x, y, pixel);
                }
            }

            let pixel: Rgb<u8> = Rgb([255, 0, 0]);
            for lm in f.landmarks {
                for x in 0..4 {
                    for y in 0..4 {
                        image.put_pixel(lm.0 + x - 2, lm.1 + y - 2, pixel);
                    }
                }
            }
        }
        image.save("resource/cabinet_processed.jpg").unwrap();
    }
}
