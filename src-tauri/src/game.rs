use image::{GenericImageView, Rgba};
use std::fs;

use serde::Serialize;

//Import Image and Frame
const WIDTH: usize = 50;
const HEIGHT: usize = 50;

#[derive(Clone, Serialize)]
pub struct Game {
    board: Vec<Vec<bool>>,
    frames: Vec<Vec<bool>>,
    current_frame: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![vec![false; WIDTH]; HEIGHT],
            frames: vec![vec![]],
            current_frame: 0,
        }
    }

    pub fn initialize_frames(&mut self) {
        let frames_path = "./frames";

        if let Ok(entries) = fs::read_dir(frames_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if let Some(image) = image::open(&path).ok() {
                        self.frames.push(self.process_image(&image));
                    }
                }
            }
        }
    }

    fn process_image(&self, img: &dyn GenericImageView<Pixel = Rgba<u8>>) -> Vec<bool> {
        let width = img.width();
        let height = img.height();

        let mut result: Vec<bool> = vec![];

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let intensity = (pixel.0[0] as f64 + pixel.0[1] as f64 + pixel.0[2] as f64) / 3.0;
                let threshold = 200.0;
                let is_white = intensity > threshold;

                result.push(is_white);
            }
        }

        result
    }

    pub fn advance_frame(&mut self) {
        self.current_frame = self.current_frame + 1;
    }

    pub fn get_current_frame(&mut self) -> Vec<bool> {
        if self.current_frame > self.frames.len() {
            return vec![];
        }

        let frame = self.frames[self.current_frame].clone();

        frame
    }
}
