use std::{fs::File, io::Write, path::Path};
use log::info;
use vector::Vector3;

mod vector;

fn main() {
    env_logger::init();

    let image_width = 640;
    let image_height = 480;
    let mut image_pixels = String::new();

    for i in 0..image_height {
        info!("scanlines remaining {}", i);
        for j in 0..image_width {
            let r = i as f64 / (image_height - 1) as f64;
            let g = j as f64 / (image_width - 1) as f64;
            let b = 0.0;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            image_pixels.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    let path = Path::new("./output/image.ppm");
    let mut _file = File::create(path);
    let image_data = format!("P3\n{} {}\n255\n{}", image_width, image_height, image_pixels);

    if let Ok(mut ppm) = _file {
        ppm.write(image_data.as_bytes()).unwrap();
    }

    let mut v = Vector3::from(4.0, 8.0, 6.0);
    // let u = Vector3::from(4.9, 7.8, 9.0);

    println!("unit: {:?}", v);
}
