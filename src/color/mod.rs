// ? INFO
// Each pixel on your computer monitor is a composite of three colors: red, green, and blue. 
// If you take those three colors and mix them in different quantities, you get just about every other color you can imagine

use std::ops;

type ColorType = u8;

struct Color {
    red: ColorType,
    green: ColorType,
    blue: ColorType,
}

impl Color {
    pub fn new(red: ColorType, green: ColorType, blue: ColorType) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
}