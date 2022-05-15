use std::env;

use image::Rgb;

use wasabi::center_face::CenterFace;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let file = args[1].clone();
    let mut image = image::open(&file).unwrap().to_rgb8();

    let scale = if args.len() > 2 {
        args[2].clone().parse().unwrap_or(1)
    } else {
        1
    };
    let wf = image.width() / 32 / scale;
    let hf = image.height() / 32 / scale;
    let cf = CenterFace::new(32 * wf, 32 * hf).unwrap();
    println!("model initialized");
    let faces = cf.detect_with_resize(&image).unwrap();
    println!("{} faces are detected", faces.len());

    let green: Rgb<u8> = Rgb([0, 255, 0]);
    let lw = 4;
    for f in faces {
        for d in 0..lw {
            for x in f.x1..f.x2 {
                image.put_pixel(x, f.y1 + d - lw / 2, green);
                image.put_pixel(x, f.y2 + d - lw / 2, green);
            }
            for y in f.y1..f.y2 {
                image.put_pixel(f.x1 + d - lw / 2, y, green);
                image.put_pixel(f.x2 + d - lw / 2, y, green);
            }
        }

        for lm in f.landmarks {
            for x in 0..lw {
                for y in 0..lw {
                    image.put_pixel(lm.0 + x - lw / 2, lm.1 + y - lw / 2, green);
                }
            }
        }
    }

    let parts: Vec<&str> = file.rsplitn(2, ".").collect();
    image.save(format!("{}_processed.{}", parts[1], parts[0])).unwrap();
}
