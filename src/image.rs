use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use png::{BitDepth, ColorType, Encoder};
use crate::grid::Grid;

pub fn save_grid(grid: &Grid<usize>, max_size: usize, out_path: PathBuf) {
    let image_data = grid.data.iter().flatten()
        .map(|&x|
            if x == max_size {
                0
            } else {
                (255. * x as f32 / max_size as f32) as u8
            }
        )
        .collect::<Vec<_>>();

    let file = File::create(out_path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, grid.data[0].len() as u32, grid.data.len() as u32);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_data).unwrap()
}