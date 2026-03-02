use std::{
    fs::{self, File},
    io::Write,
};

use image::RgbImage;

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let width = 256;
    let height = 256;
    let max_val = 255;
    let test_raster = {
        let mut raster = Vec::new();
        for _ in 0..width * height {
            raster.push(RGB {
                red: 255,
                green: 0,
                blue: 0,
            });
        }
        raster
    };
    write_bmp("test", width, height, &test_raster);
}

fn write_bmp(name: &str, width: u32, height: u32, raster: &[RGB]) {
    let bytes: Vec<u8> = raster
        .iter()
        .flat_map(|pixel| [pixel.red, pixel.green, pixel.blue])
        .collect();
    let img = RgbImage::from_raw(width, height, bytes).unwrap();
    fs::create_dir_all(".output").unwrap();
    let path = format!(".output/{}.bmp", name);
    img.save(path).unwrap();
}
