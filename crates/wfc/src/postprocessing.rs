use image::{ImageBuffer, Rgb, RgbImage};

use crate::model::pattern::Pattern;

pub fn reconstruct_image(
    output: &[u16],
    grid_width: u32,
    grid_height: u32,
    patterns: &[Pattern],
    palette: &[Rgb<u8>],
    pattern_width: u32,
    pattern_height: u32,
) -> RgbImage {
    let img_width = grid_width + pattern_width - 1;
    let img_height = grid_height + pattern_height - 1;
    let mut img = ImageBuffer::new(img_width, img_height);

    for gy in 0..grid_height {
        for gx in 0..grid_width {
            let grid_idx = (gx + gy * grid_width) as usize;
            let pattern = &patterns[output[grid_idx] as usize];
            for py in 0..pattern_height {
                for px in 0..pattern_width {
                    let img_x = gx + px;
                    let img_y = gy + py;
                    let sample_idx = (px + py * pattern_width) as usize;
                    let color = palette[pattern.samples[sample_idx] as usize];
                    img.put_pixel(img_x, img_y, color);
                }
            }
        }
    }
    img
}
