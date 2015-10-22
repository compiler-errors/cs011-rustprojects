//TODO: Implement a simple image array which I can use to output the scene to a ppm file...
//File IO, rust arrays, proper usage of accessors and mutators...

use geom::color::Color;

pub struct Image {
    width: i32,
    height: i32,
    image: Vec<Vec<Color>>
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        assert!(width > 0);
        assert!(height > 0);

        let mut buffer = Vec::new();
        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                row.push(Color::black());
            }
            buffer.push(row);
        }

        Image {width: width, height: height, image: buffer}
    }
}
