use std::io::prelude::*;
use std::fs::File;

use geom::color::Color;

// An Image struct stores a two-dimensional Vec of Color
pub struct Image {
    width: i32,
    height: i32,
    image: Vec<Vec<Color>>
}

impl Image {
    /// Allocates an image filled with black with a
    /// size given by width and height.
    pub fn new(width: i32, height: i32) -> Image {
        // We can't have an image with no pixels.
        assert!(width > 0);
        assert!(height > 0);

        // Rust is very smart!
        // It can infer the generic type of Vec by usage
        // in later lines.
        let mut buffer = Vec::new();
        // We denote unused variable with _ so the
        // compiler doesn't complain.
        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                // At this point, the "row" variable can
                // infer the type of Vec to be Vec<Color>.
                row.push(Color::black());
            }
            buffer.push(row);
        }

        Image {width: width, height: height, image: buffer}
    }

    /// Replaces the color at (x, y) with the new color provided
    pub fn set_color(&mut self, x: i32, y: i32, color: Color) {
        assert!(x >= 0 && x < self.width, "x out of bounds!");
        assert!(y >= 0 && y < self.height, "y out of bounds!");

        self.image[x as usize][y as usize] = color;
    }

    /// Save the image struct as a ppm (portable bitmap) image file.
    ///
    /// The file is saved as the filename parameter, without the .ppm
    /// file extension added onto the end.
    pub fn save(&self, filename: &str) {
        //TODO: using unwrap is bad.
        //TODO: check for file extension?
        let mut file = File::create(filename).unwrap();
        write!(file, "P3\n").unwrap();
        write!(file, "{} {}\n", self.width, self.height).unwrap();
        write!(file, "255\n").unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                //the i32 value for x and y need to be cast to usize
                let color = self.image[x as usize][y as usize].clamp();
                let r = (color.r * 255.0) as i32;
                let g = (color.g * 255.0) as i32;
                let b = (color.b * 255.0) as i32;
                write!(file, "{} {} {} ", r, g, b).unwrap();
            }
            write!(file, "\n").unwrap();
        }
    }
}
