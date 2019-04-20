// ? INFO
// Each piredel on your computer monitor is a composite of three colors: red, green, and blue. 
// If you take those three colors and mired them in different quantities, you get just about every other color you can imagine

use std::ops;

use crate::utils::NumberUtils;

type ColorType = f32;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    pub red: ColorType,
    pub green: ColorType,
    pub blue: ColorType,
}

impl Color {
    
    pub fn new(red: ColorType, green: ColorType, blue: ColorType) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn to_ppm_value(value: ColorType) -> ColorType {
        let value = ColorType::round(255. * value);
        match value {
            n if n < 0. => 0.,
            n if n > 255. => 255.,
            n => n
        }
    }    

    pub fn is_equal_to(self, other_color: Color) -> bool {
        NumberUtils::compare_floats_32(self.red, other_color.red) &&
        NumberUtils::compare_floats_32(self.green, other_color.green) &&
        NumberUtils::compare_floats_32(self.blue, other_color.blue)
    }

    pub fn to_string(&self) -> String {
        let red = Color::to_ppm_value(self.red);
        let green = Color::to_ppm_value(self.green);
        let blue = Color::to_ppm_value(self.blue);
        format!("{} {} {}", red, green, blue)
    }

    pub fn black() -> Self {
        Color {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    pub fn red() -> Self {
        Color {
            red: 1.,
            green: 0.,
            blue: 0.,
        }
    }

    pub fn green() -> Self {
        Color {
            red: 0.,
            green: 1.,
            blue: 0.,
        }
    }

    pub fn blue() -> Self {
        Color {
            red: 0.,
            green: 0.,
            blue: 1.,
        }
    }

    pub fn white() -> Self {
        Color {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    }
    
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        Color {
            red: self.red + color.red,
            green: self.green + color.green,
            blue: self.blue + color.blue,
        }
    }

}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, color: Color) -> Color {
        Color {
            red: self.red - color.red,
            green: self.green - color.green,
            blue: self.blue - color.blue,
        }
    }
}

impl ops::Div<ColorType> for Color {
    type Output = Color;

    fn div(self, scalar: ColorType) -> Color {
        Color {
            red: self.red / scalar,
            green: self.green / scalar,
            blue: self.blue / scalar,
        }
    }

}

impl ops::Mul<ColorType> for Color {
    type Output = Color;

    fn mul(self, scalar: ColorType) -> Self {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

// Hadamard product / Schur product / blending colors
impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other_color: Color) -> Self {
        Color {
            red: self.red * other_color.red,
            green: self.green * other_color.green,
            blue: self.blue * other_color.blue,
        }
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_new_color() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color, Color { red: -0.5, green: 0.4, blue: 1.7 });
    }

    #[test]
    fn test_add_colors() {
        let color_1 = Color::new(0.9, 0.6, 0.75);
        let color_2 = Color::new(0.7, 0.1, 0.25);

        let new_color = color_1 + color_2;
        assert_eq!(new_color.is_equal_to(Color { red: 1.6, green: 0.7, blue: 1.0 }), true);
    }

    #[test]
    fn test_sub_colors() {
        let color_1 = Color::new(0.9, 0.6, 0.75);
        let color_2 = Color::new(0.7, 0.1, 0.25);

        let new_color = color_1 - color_2;
        assert_eq!(new_color.is_equal_to(Color { red: 0.2, green: 0.5, blue: 0.5}), true);
    }

    #[test]
    fn test_multiply_by_sclar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let scalar = 2_f32;

        let new_color = color * scalar;

        assert_eq!(new_color.is_equal_to(Color { red: 0.4, green: 0.6, blue: 0.8 }), true);
    }

    #[test]
    fn test_hadamard_product() {
        let color_1 = Color::new(1., 0.2, 0.4);
        let color_2 = Color::new(0.9, 1., 0.1);

        let new_color = color_1 * color_2;
        assert_eq!(new_color.is_equal_to(Color { red: 0.9, green: 0.2, blue: 0.04}), true);
        
    }

}