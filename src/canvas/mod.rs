use std::fs::File;
use std::io::prelude::*;

use crate::color::Color;

type Size = usize;

pub struct Canvas {
    width: Size,
    height: Size,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: Size, height: Size) -> Self {
        Canvas {
            width: width,
            height: height,
            pixels: vec![vec![Color::black(); width]; height]
        }
    }

    pub fn new_with_color(width: Size, height: Size, color: Color) -> Self {
        Canvas {
            width: width,
            height: height,
            pixels: vec![vec![color; width]; height]
        }
    }

    pub fn get_pixel_at(&self, x: Size, y: Size) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel_at(&mut self, x: Size, y: Size, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn format_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn format_ppm_data(&self) -> String {
        let mut data = String::new();

        for colors in &self.pixels {
            let formated_vector: Vec<String> = colors.iter().map(|color: &Color| color.to_string()).collect();
            data += &(Canvas::format_vector(formated_vector));
        }

        data
    }

    fn format_vector(vec: Vec<String>) -> String {
        let color_row = vec.join(" ") + "\n";

        match color_row.len() {
            n if n > 70 => {
                let mut formated_row = String::new();
                let mut prev = String::new();
                let vec = color_row.split(" ").collect::<Vec<&str>>();

                for v in &vec {
                    let temp = format!("{} {}", prev, v);

                    if temp.len() > 70 {
                        formated_row += &format!("{}\n", prev.trim());
                        prev = format!("{}", v);
                    } else {
                        prev = temp;
                    }
                }

                format!("{}{}", formated_row, prev)
            },
            _ => color_row
        }
    }

    pub fn save_to_disk(&self) -> std::io::Result<()> {
        let header = self.format_ppm_header();
        let data = self.format_ppm_data();

        let mut file = File::create("image.ppm")?;        

        file.write(header.as_bytes());
        file.write(data.as_bytes());

        Ok(())
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_new_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.pixels.len(), 20);
        assert_eq!(canvas.pixels[0][0], Color::black());
        assert_eq!(canvas.pixels[19][9], Color::black());
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::red();
        
        assert_eq!(canvas.get_pixel_at(2, 3), Color::black());

        canvas.write_pixel_at(2, 3, red);
        assert_eq!(canvas.get_pixel_at(2, 3), Color::red());
    }

    #[test]
    fn test_format_ppm_header() {
        let format_example = "P3\n10 20\n255\n";
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.format_ppm_header(), format_example);
    }

    #[test]
    fn test_format_ppm_data() {
        let format_example = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        let mut canvas = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        canvas.write_pixel_at(0, 0, c1);
        canvas.write_pixel_at(2, 1, c2);
        canvas.write_pixel_at(4, 2, c3);

        assert_eq!(canvas.format_ppm_data(), format_example);
        
    }

    #[test]
    fn test_format_long_data() {
        let format_example = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        let color = Color::new(1., 0.8, 0.6);
        let canvas = Canvas::new_with_color(10, 2, color);

        assert_eq!(canvas.format_ppm_data(), format_example);

    }
}

