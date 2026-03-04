use std::vec;

use image::{DynamicImage, ImageReader, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/test_input.bmp")?.decode()?;
    overlap_model(input_img);
    Ok(())
}

fn rgb8_to_string(pixel: Rgb<u8>) -> String {
    format!("{},{},{}", pixel[0], pixel[1], pixel[2])
}

fn sample_dynamic_image(img: &DynamicImage) -> (u32, u32, Vec<u16>, Vec<String>) {
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut sample: Vec<u16> = vec![0; (height * width) as usize];
    let mut colors: Vec<String> = vec![];
    for (x, y, pixel) in img.enumerate_pixels() {
        let mut k = 0;
        let pixel_string = rgb8_to_string(*pixel);
        for i in 0..colors.len() {
            k = i;
            if colors[i] == pixel_string {
                break;
            }
        }
        if k + 1 >= colors.len() {
            colors.push(pixel_string);
        }
        let index = x + y * width;
        sample[index as usize] = k as u16;
    }
    (width, height, sample, colors)
}

fn overlap_model(img: DynamicImage) {
    let (width, height, sample, _color_map) = sample_dynamic_image(&img);
    print_sampled_input(width, height, &sample);
}

fn print_sampled_input(width: u32, height: u32, sample_arr: &Vec<u16>) {
    println!("Sampled input:");
    for i in 0..height {
        for j in 0..width {
            let index = j + i * height;
            print!("{} ", sample_arr[index as usize]);
        }
        println!();
    }
}
