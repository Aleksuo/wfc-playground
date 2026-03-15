use image::{ImageBuffer, Rgb, RgbImage};

pub fn reconstruct_image(
    output: &Vec<u16>,
    width: u32,
    height: u32,
    palette: &Vec<Rgb<u8>>,
) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let idx = (x + y * width) as usize;
            let color = palette[output[idx] as usize];
            img.put_pixel(x, y, color);
        }
    }
    img
}
