use std::env;

use image::Rgb;

use wasabi::center_face::CenterFace;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let cf = CenterFace::new(32 * 15, 32 * 20).unwrap();
    let mut image = image::open(args[1].clone()).unwrap().to_rgb8();
    let faces = cf.detect_with_resize(&image).unwrap();

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
    image.save("resource/processed.jpg").unwrap();
}
