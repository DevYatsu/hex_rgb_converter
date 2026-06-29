use std::fmt;

extern crate custom_error;
use custom_error::custom_error;

custom_error! { pub ConvertionError
    HexToRgbError{msg: String} = "{msg}",
    RgbToHexError{msg: String} = "{msg}",
}
custom_error! { pub InitializationError
    InvalidRgb{at: String} = "Invalid rgb color at '{at}'",
    InvalidHex{at: String} = "Invalid hex color at '{at}'",
}
custom_error! { pub InvalidParameterError
    InvalidColorError{at: String} = "The chosen color does not exist. Color may be either 'r', 'g' or 'b': at {at}"
}


pub struct Color;

impl Color {
    pub fn hex(color: &str) -> HexColor {
        HexColor::new(color)
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor::new(r, g, b)
    }
    pub fn by_name(name: &str) -> HexColor
    {
        let color: &str = match name {
            "yellow" => "FFFF00",
            "white" => "FFFFFF",
            "seashell" => "FFF5EE",
            "red" => "FF0000",
            "lime" => "00FF00",
            "blue" => "0000FF",
            "cyan" => "00FFFF",
            "magenta" => "FF00FF",
            "silver" => "C0C0C0",
            "gray" => "808080",
            "maroon" => "800000",
            "olive" => "808000",
            "green" => "008000",
            "purple" => "800080",
            "teal" => "008080",
            "navy" => "000080",
            "orange" => "FFA500",
            "tomato" => "FF6347",
            "coral" => "FF7F50",
            "pink" => "FFC0CB",
            "gold" => "FFD700",
            "sky" => "87CEEB",
            "lavender" => "E6E6FA",
            "amber" => "FFBF00", 
            "apricot" => "FBCEB1",
            "black" | _  => "000000",
        };

        HexColor::new(&color)
    }
}

#[derive(Debug)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub enum Colors {
    Red(u8),
    Green(u8), 
    Blue(u8),
    All(u8)
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    fn get_hex_equivalent(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b).to_lowercase()
    }
    pub fn to_hex(&self) -> HexColor {
        let color: &String = &self.get_hex_equivalent();

        HexColor::new(&color)
    }
    pub fn is_equal(&self, other: &HexColor) -> bool {
        self.get_hex_equivalent() == other.color
    }
    pub fn print(&self) {
        println!("({}, {}, {})", self.r, self.g, self.b)
    }
    pub fn set_red(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }

    pub fn set_green(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }
    pub fn set_blue(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }
    pub fn set_color(&mut self, color: Colors) -> &mut Self {
        match color {
            Colors::Red(value) => self.r = value,
            Colors::Green(value) => self.g = value,
            Colors::Blue(value) => self.b = value,
            Colors::All(value) => {
                // 'a' for 'all'
                self.r = value;
                self.g = value;
                self.b = value;
            }
        };
        self
    }
    fn tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)    
    }
    pub fn are_equal(one_instance: &RgbColor, other_instance: &RgbColor) -> bool {
        one_instance.tuple() == other_instance.tuple()
    }
}

#[derive(Debug)]
pub struct HexColor {
    color: String,
}

impl HexColor {
    pub fn new(color: &str) -> Self {
        let mut color: String = String::from(color);

        if color.starts_with("#") {
            color.remove(0);
        }
        if color.len() > 6 {
            panic!("{}", InitializationError::InvalidHex {
                at: format!("{}", color),
            }
            .to_string())
        }
        for c in color.chars() {
            if !c.is_digit(16) {
                panic!("{}", InitializationError::InvalidHex {
                    at: format!("{}", color),
                }
                .to_string())
            }
        }

        for _ in 0..6 - color.len() {
            color.insert(0, '0');
        }

        color = color.to_lowercase();

        Self { color }
    }
    fn get_rgb_equivalent(&self) -> (u8, u8, u8) {
        let r = u8::from_str_radix(&self.color[0..2], 16)
            .map_err(|_| "Invalid hex color")
            .unwrap();
        let g = u8::from_str_radix(&self.color[2..4], 16)
            .map_err(|_| "Invalid hex color")
            .unwrap();
        let b = u8::from_str_radix(&self.color[4..6], 16)
            .map_err(|_| "Invalid hex color")
            .unwrap();

        (r, g, b)
    }
    pub fn to_rgb(&self) -> RgbColor {
        let (r, g, b) = self.get_rgb_equivalent();

        RgbColor::new(r, g, b)
    }
    pub fn is_equal(&self, other: &RgbColor) -> bool {
        self.get_rgb_equivalent() == (other.r, other.g, other.g)
    }
    pub fn print(&self) {
        println!("#{}", self.color)
    }
    pub fn are_equal(one_instance: &HexColor, other_instance: &HexColor) -> bool {
        one_instance.color == other_instance.color
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.color)
    }
}
impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.r, self.g, self.b)
    }
}
