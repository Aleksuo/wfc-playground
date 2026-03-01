use std::{fs::File, io::Write};

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
    write_ppm("test", width, height, max_val, test_raster);
}

fn write_ppm(name: &str, width: u32, height: u32, max_val: u16, raster: Vec<RGB>) {
    let header_row = "P3\n".to_string();
    let width_height_row = format!("{} {}\n", width, height);
    let max_val_row = format!("{}\n", max_val);
    let raster_row: String = {
        let mut rows = Vec::new();
        for i in 0..height {
            let mut row = Vec::new();
            for j in 0..width {
                let index = i * width + j;
                let raster_color = &raster[index as usize];
                row.push(format!(
                    "{} {} {}",
                    raster_color.red, raster_color.green, raster_color.blue
                ));
            }
            rows.push(row.join(" "));
        }
        rows.join("\n")
    };
    let output = [header_row, width_height_row, max_val_row, raster_row].join("");
    let path = format!(".output/{}.ppm", name);
    let mut output_file = File::create(path).unwrap();
    write!(output_file, "{}", output);
}
