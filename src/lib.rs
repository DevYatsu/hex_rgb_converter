//! Utilities for working with RGB and hex color representations.
//!
//! This crate provides types for creating, converting, and comparing colors
//! expressed as RGB tuples or hexadecimal strings. It also includes a small
//! collection of named colors and builder-style mutation methods.
//!
//! # Quick start
//!
//! ```
//! use hex_rgb_converter::{Color, RgbColor, HexColor};
//!
//! // Create a color from a hex string.
//! let hex = Color::hex("1e90ff");
//!
//! // Create a color from RGB components.
//! let rgb = Color::rgb(30, 144, 255);
//!
//! // Convert between representations.
//! assert_eq!(hex.to_rgb().to_hex().to_string(), hex.to_string());
//! ```
//!
//! # Named colours
//!
//! A small set of common colour names is recognised by [`Color::by_name`]:
//!
//! ```
//! use hex_rgb_converter::Color;
//!
//! let orange = Color::by_name("orange");
//! assert_eq!(orange.to_string(), "#ffa500");
//! ```

use std::fmt;

extern crate custom_error;
use custom_error::custom_error;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

custom_error! { pub ConvertionError
    /// Returned when a conversion into a hex representation fails.
    HexToRgbError{msg: String} = "{msg}",
    /// Returned when a conversion into an RGB representation fails.
    RgbToHexError{msg: String} = "{msg}",
}
custom_error! { pub InitializationError
    /// Provided string could not be parsed as an RGB value.
    InvalidRgb{at: String} = "Invalid rgb color at '{at}'",
    /// Provided string could not be parsed as a hex colour.
    InvalidHex{at: String} = "Invalid hex color at '{at}'",
}
custom_error! { pub InvalidParameterError
    /// An unrecognised colour name was supplied where `r`, `g` or `b` was expected.
    InvalidColorError{at: String} = "The chosen color does not exist. Color may be either 'r', 'g' or 'b': at {at}"
}

// ---------------------------------------------------------------------------
// Color factory
// ---------------------------------------------------------------------------

/// Entry-point for constructing colour values.
///
/// `Color` is a unit struct that serves as a factory / namespace.  Every
/// colour is ultimately represented as either an [`RgbColor`] or a
/// [`HexColor`].
///
/// # Examples
///
/// ```
/// use hex_rgb_converter::Color;
///
/// let hex = Color::hex("c0ffee");
/// let rgb = Color::rgb(192, 255, 238);
/// ```
pub struct Color;

impl Color {
    /// Parse a hexadecimal string into a [`HexColor`].
    ///
    /// An optional leading `#` is stripped automatically.  Values shorter than
    /// 6 hex digits are left-padded with zeroes.
    ///
    /// # Panics
    ///
    /// Panics if the input is longer than 6 hex digits or contains any
    /// non-hexadecimal character.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let c = Color::hex("ff7f50");
    /// assert_eq!(c.to_string(), "#ff7f50");
    /// ```
    pub fn hex(color: &str) -> HexColor {
        HexColor::new(color)
    }

    /// Create an [`RgbColor`] from its red, green and blue components.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let c = Color::rgb(255, 127, 80);
    /// assert_eq!(c.to_string(), "(255,127,80)");
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor::new(r, g, b)
    }

    /// Look up a colour by its common English name and return a [`HexColor`].
    ///
    /// The match is case-sensitive.  If the name is not recognised the
    /// fallback is black (`000000`).
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let c = Color::by_name("coral");
    /// assert_eq!(c.to_string(), "#ff7f50");
    /// ```
    pub fn by_name(name: &str) -> HexColor {
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
            _ => "000000",
        };

        HexColor::new(color)
    }
}

// ---------------------------------------------------------------------------
// RgbColor
// ---------------------------------------------------------------------------

/// A colour represented by its red, green and blue components.
///
/// Each component is an unsigned 8-bit integer (0–255).  `RgbColor` can be
/// converted to a [`HexColor`] via [`to_hex`](RgbColor::to_hex) and inspected
/// directly through its public fields.
///
/// # Examples
///
/// ```
/// use hex_rgb_converter::RgbColor;
///
/// let mut c = RgbColor::new(30, 144, 255);
/// assert_eq!(c.r, 30);
/// assert_eq!(c.g, 144);
///
/// // Builder-style mutation.
/// c.set_red(255);
/// assert_eq!(c.r, 255);
/// ```
#[derive(Debug)]
pub struct RgbColor {
    /// Red component (0–255).
    pub r: u8,
    /// Green component (0–255).
    pub g: u8,
    /// Blue component (0–255).
    pub b: u8,
}

/// Selector for individual or all colour channels.
///
/// Used with [`RgbColor::set_color`] to modify one channel or set all three
/// to the same value at once.
pub enum Colors {
    /// Target the red channel with the given value.
    Red(u8),
    /// Target the green channel with the given value.
    Green(u8),
    /// Target the blue channel with the given value.
    Blue(u8),
    /// Set all three channels to the same value.
    All(u8),
}

impl RgbColor {
    /// Construct a new `RgbColor` from individual RGB components.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::RgbColor;
    ///
    /// let c = RgbColor::new(255, 0, 128);
    /// assert_eq!(c.to_string(), "(255,0,128)");
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Format the internal RGB values as a zero-padded, lower-case hex string
    /// (without `#` prefix).
    fn get_hex_equivalent(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b).to_lowercase()
    }

    /// Convert this `RgbColor` to its [`HexColor`] representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::RgbColor;
    ///
    /// let rgb = RgbColor::new(255, 165, 0);
    /// let hex = rgb.to_hex();
    /// assert_eq!(hex.to_string(), "#ffa500");
    /// ```
    pub fn to_hex(&self) -> HexColor {
        let color: &String = &self.get_hex_equivalent();

        HexColor::new(color)
    }

    /// Check whether this `RgbColor` is visually equal to a given [`HexColor`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::{Color, RgbColor};
    ///
    /// let rgb = Color::rgb(255, 165, 0);
    /// let hex = Color::hex("ffa500");
    /// assert!(rgb.is_equal(&hex));
    /// ```
    pub fn is_equal(&self, other: &HexColor) -> bool {
        self.get_hex_equivalent() == other.color
    }

    /// Print the colour as an `(r,g,b)` triple to stdout.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hex_rgb_converter::Color;
    ///
    /// Color::rgb(10, 20, 30).print();
    /// // Output: (10, 20, 30)
    /// ```
    pub fn print(&self) {
        println!("({}, {}, {})", self.r, self.g, self.b)
    }

    /// Set the red channel and return `&mut self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let mut c = Color::rgb(0, 0, 0);
    /// c.set_red(128);
    /// assert_eq!(c.r, 128);
    /// ```
    pub fn set_red(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }

    /// Set the green channel and return `&mut self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let mut c = Color::rgb(0, 0, 0);
    /// c.set_green(128);
    /// assert_eq!(c.g, 128);
    /// ```
    pub fn set_green(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }

    /// Set the blue channel and return `&mut self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let mut c = Color::rgb(0, 0, 0);
    /// c.set_blue(128);
    /// assert_eq!(c.b, 128);
    /// ```
    pub fn set_blue(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }

    /// Set one or all colour channels via a [`Colors`] selector.
    ///
    /// This is a convenience wrapper around [`set_red`](RgbColor::set_red),
    /// [`set_green`](RgbColor::set_green) and
    /// [`set_blue`](RgbColor::set_blue) that works with a single enum
    /// variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::{Color, Colors};
    ///
    /// let mut c = Color::rgb(100, 100, 100);
    /// c.set_color(Colors::Red(255));
    /// assert_eq!(c.r, 255);
    ///
    /// c.set_color(Colors::All(0));
    /// assert_eq!(c.to_string(), "(0,0,0)");
    /// ```
    pub fn set_color(&mut self, color: Colors) -> &mut Self {
        match color {
            Colors::Red(value) => self.r = value,
            Colors::Green(value) => self.g = value,
            Colors::Blue(value) => self.b = value,
            Colors::All(value) => {
                self.r = value;
                self.g = value;
                self.b = value;
            }
        };
        self
    }

    /// Return the RGB components as a tuple.
    fn tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Check whether two `RgbColor` values are equal in all channels.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::RgbColor;
    ///
    /// let a = RgbColor::new(10, 20, 30);
    /// let b = RgbColor::new(10, 20, 30);
    /// assert!(RgbColor::are_equal(&a, &b));
    /// ```
    pub fn are_equal(one_instance: &RgbColor, other_instance: &RgbColor) -> bool {
        one_instance.tuple() == other_instance.tuple()
    }
}

// ---------------------------------------------------------------------------
// HexColor
// ---------------------------------------------------------------------------

/// A colour represented by a six-digit hexadecimal string (without `#`).
///
/// The internal string is always stored in lower case and left-padded to six
/// digits.  `HexColor` can be converted to an [`RgbColor`] via
/// [`to_rgb`](HexColor::to_rgb).
///
/// # Examples
///
/// ```
/// use hex_rgb_converter::HexColor;
///
/// let c = HexColor::new("ff7f50");
/// assert_eq!(c.to_string(), "#ff7f50");
/// ```
#[derive(Debug)]
pub struct HexColor {
    /// The normalized six-digit hex string (lower-case, no `#`).
    color: String,
}

impl HexColor {
    /// Parse a hex string into a `HexColor`.
    ///
    /// An optional leading `#` is stripped automatically.  Inputs shorter than
    /// six hex digits are left-padded with `0`.
    ///
    /// # Panics
    ///
    /// Panics if the input (after stripping `#`) is longer than six characters
    /// or contains any character that is not a valid hex digit (0–9, a–f,
    /// A–F).
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::HexColor;
    ///
    /// let c = HexColor::new("#a52a2a");  // leading # is ok
    /// assert_eq!(c.to_string(), "#a52a2a");
    /// ```
    pub fn new(color: &str) -> Self {
        let mut color: String = String::from(color);

        if color.starts_with("#") {
            color.remove(0);
        }
        if color.len() > 6 {
            panic!(
                "{}",
                InitializationError::InvalidHex {
                    at: color.clone(),
                }
                .to_string()
            )
        }
        for c in color.chars() {
            if !c.is_ascii_hexdigit() {
                panic!(
                    "{}",
                    InitializationError::InvalidHex {
                        at: color.clone(),
                    }
                    .to_string()
                )
            }
        }

        for _ in 0..6 - color.len() {
            color.insert(0, '0');
        }

        color = color.to_lowercase();

        Self { color }
    }

    /// Decompose the hex string into its red, green and blue components.
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

    /// Convert this `HexColor` to its [`RgbColor`] representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let hex = Color::hex("ffa500");
    /// let rgb = hex.to_rgb();
    /// assert_eq!(rgb.to_string(), "(255,165,0)");
    /// ```
    pub fn to_rgb(&self) -> RgbColor {
        let (r, g, b) = self.get_rgb_equivalent();

        RgbColor::new(r, g, b)
    }

    /// Check whether this `HexColor` is visually equal to a given [`RgbColor`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::Color;
    ///
    /// let hex = Color::hex("ffa500");
    /// let rgb = Color::rgb(255, 165, 0);
    /// assert!(hex.is_equal(&rgb));
    /// ```
    pub fn is_equal(&self, other: &RgbColor) -> bool {
        self.get_rgb_equivalent() == (other.r, other.g, other.b)
    }

    /// Print the colour as `#rrggbb` to stdout.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hex_rgb_converter::Color;
    ///
    /// Color::hex("c0ffee").print();
    /// // Output: #c0ffee
    /// ```
    pub fn print(&self) {
        println!("#{}", self.color)
    }

    /// Check whether two `HexColor` values represent the same colour.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_rgb_converter::HexColor;
    ///
    /// let a = HexColor::new("c0ffee");
    /// let b = HexColor::new("c0ffee");
    /// assert!(HexColor::are_equal(&a, &b));
    /// ```
    pub fn are_equal(one_instance: &HexColor, other_instance: &HexColor) -> bool {
        one_instance.color == other_instance.color
    }
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

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

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Color::hex
    // -----------------------------------------------------------------------

    #[test]
    fn hex_parses_six_digit() {
        let c = Color::hex("ff7f50");
        assert_eq!(c.to_string(), "#ff7f50");
    }

    #[test]
    fn hex_strips_leading_hash() {
        let c = Color::hex("#ff7f50");
        assert_eq!(c.to_string(), "#ff7f50");
    }

    #[test]
    fn hex_pads_short_input() {
        let c = Color::hex("fff");
        assert_eq!(c.to_string(), "#000fff");
    }

    #[test]
    fn hex_pads_single_char() {
        let c = Color::hex("0");
        assert_eq!(c.to_string(), "#000000");
    }

    #[test]
    fn hex_empty_pads_to_all_zeroes() {
        let c = Color::hex("");
        assert_eq!(c.to_string(), "#000000");
    }

    #[test]
    fn hex_lowercases_uppercase() {
        let c = Color::hex("FFF");
        assert_eq!(c.to_string(), "#000fff");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn hex_rejects_overlong() {
        Color::hex("fffffff");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn hex_rejects_non_hex() {
        Color::hex("ffxx00");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn hex_rejects_special_chars() {
        Color::hex("ff-00!");
    }

    // -----------------------------------------------------------------------
    // Color::rgb
    // -----------------------------------------------------------------------

    #[test]
    fn rgb_creates_color() {
        let c = Color::rgb(255, 128, 64);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 64);
    }

    #[test]
    fn rgb_zero_values() {
        let c = Color::rgb(0, 0, 0);
        assert_eq!(c.to_string(), "(0,0,0)");
    }

    #[test]
    fn rgb_max_values() {
        let c = Color::rgb(255, 255, 255);
        assert_eq!(c.to_string(), "(255,255,255)");
    }

    // -----------------------------------------------------------------------
    // Color::by_name
    // -----------------------------------------------------------------------

    #[test]
    fn by_name_known_colors() {
        assert_eq!(Color::by_name("red").to_string(), "#ff0000");
        assert_eq!(Color::by_name("lime").to_string(), "#00ff00");
        assert_eq!(Color::by_name("blue").to_string(), "#0000ff");
        assert_eq!(Color::by_name("white").to_string(), "#ffffff");
        assert_eq!(Color::by_name("orange").to_string(), "#ffa500");
    }

    #[test]
    fn by_name_case_sensitive() {
        assert_eq!(Color::by_name("RED").to_string(), "#000000");
    }

    #[test]
    fn by_name_unknown_falls_back_to_black() {
        assert_eq!(Color::by_name("nonexistent").to_string(), "#000000");
    }

    // -----------------------------------------------------------------------
    // RgbColor
    // -----------------------------------------------------------------------

    #[test]
    fn rgb_round_trips_to_hex_and_back() {
        let rgb = RgbColor::new(30, 144, 255);
        let hex = rgb.to_hex();
        let back = hex.to_rgb();
        assert_eq!(rgb.r, back.r);
        assert_eq!(rgb.g, back.g);
        assert_eq!(rgb.b, back.b);
    }

    #[test]
    fn rgb_to_hex_conversion() {
        let rgb = RgbColor::new(255, 165, 0);
        assert_eq!(rgb.to_hex().to_string(), "#ffa500");
    }

    #[test]
    fn rgb_is_equal_compares_with_hex() {
        let rgb = Color::rgb(255, 165, 0);
        let hex = Color::hex("ffa500");
        assert!(rgb.is_equal(&hex));
        assert!(!rgb.is_equal(&Color::hex("ffffff")));
    }

    #[test]
    fn rgb_set_red_chaining() {
        let mut c = RgbColor::new(0, 0, 0);
        c.set_red(100);
        assert_eq!(c.r, 100);
    }

    #[test]
    fn rgb_set_green_chaining() {
        let mut c = RgbColor::new(0, 0, 0);
        c.set_green(150);
        assert_eq!(c.g, 150);
    }

    #[test]
    fn rgb_set_blue_chaining() {
        let mut c = RgbColor::new(0, 0, 0);
        c.set_blue(200);
        assert_eq!(c.b, 200);
    }

    #[test]
    fn rgb_set_all_returns_self_for_chaining() {
        let mut c = RgbColor::new(0, 0, 0);
        let ptr: *const RgbColor = c.set_red(1).set_green(2).set_blue(3);
        assert_eq!(c.r, 1);
        assert_eq!(c.g, 2);
        assert_eq!(c.b, 3);
        // pointer check: chaining returns &mut self
        assert_eq!(ptr, &c as *const RgbColor);
    }

    #[test]
    fn rgb_set_color_red_variant() {
        let mut c = RgbColor::new(10, 20, 30);
        c.set_color(Colors::Red(255));
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 20);
        assert_eq!(c.b, 30);
    }

    #[test]
    fn rgb_set_color_green_variant() {
        let mut c = RgbColor::new(10, 20, 30);
        c.set_color(Colors::Green(255));
        assert_eq!(c.r, 10);
        assert_eq!(c.g, 255);
        assert_eq!(c.b, 30);
    }

    #[test]
    fn rgb_set_color_blue_variant() {
        let mut c = RgbColor::new(10, 20, 30);
        c.set_color(Colors::Blue(255));
        assert_eq!(c.r, 10);
        assert_eq!(c.g, 20);
        assert_eq!(c.b, 255);
    }

    #[test]
    fn rgb_set_color_all_variant() {
        let mut c = RgbColor::new(10, 20, 30);
        c.set_color(Colors::All(128));
        assert_eq!(c.r, 128);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 128);
    }

    #[test]
    fn rgb_set_color_returns_self() {
        let mut c = RgbColor::new(0, 0, 0);
        c.set_color(Colors::All(1));
        assert_eq!(c.r, 1);
        assert_eq!(c.g, 1);
        assert_eq!(c.b, 1);
    }

    #[test]
    fn rgb_are_equal_identical() {
        let a = RgbColor::new(10, 20, 30);
        let b = RgbColor::new(10, 20, 30);
        assert!(RgbColor::are_equal(&a, &b));
    }

    #[test]
    fn rgb_are_equal_different() {
        let a = RgbColor::new(10, 20, 30);
        let b = RgbColor::new(99, 20, 30);
        assert!(!RgbColor::are_equal(&a, &b));
    }

    #[test]
    fn rgb_display_format() {
        let c = RgbColor::new(1, 2, 3);
        assert_eq!(format!("{}", c), "(1,2,3)");
    }

    // -----------------------------------------------------------------------
    // HexColor
    // -----------------------------------------------------------------------

    #[test]
    fn hex_parses_with_hash() {
        let c = HexColor::new("#a52a2a");
        assert_eq!(c.to_string(), "#a52a2a");
    }

    #[test]
    fn hex_pads_to_six_digits() {
        let c = HexColor::new("abc");
        assert_eq!(c.to_string(), "#000abc");
    }

    #[test]
    fn hex_stores_lowercase() {
        let c = HexColor::new("ABC123");
        assert_eq!(c.to_string(), "#abc123");
    }

    #[test]
    fn hex_to_rgb_conversion() {
        let hex = HexColor::new("ffa500");
        let rgb = hex.to_rgb();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 165);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn hex_is_equal_compares_with_rgb() {
        let hex = Color::hex("ffa500");
        let rgb = Color::rgb(255, 165, 0);
        assert!(hex.is_equal(&rgb));
        assert!(!hex.is_equal(&Color::rgb(0, 0, 0)));
    }

    #[test]
    fn hex_are_equal_identical() {
        let a = HexColor::new("c0ffee");
        let b = HexColor::new("c0ffee");
        assert!(HexColor::are_equal(&a, &b));
    }

    #[test]
    fn hex_are_equal_different() {
        let a = HexColor::new("c0ffee");
        let b = HexColor::new("ffffff");
        assert!(!HexColor::are_equal(&a, &b));
    }

    #[test]
    fn hex_display_format() {
        let c = HexColor::new("c0ffee");
        assert_eq!(format!("{}", c), "#c0ffee");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn hex_new_panics_on_garbage() {
        HexColor::new("zzzzzz");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn hex_new_panics_on_too_long() {
        HexColor::new("1234567");
    }

    // -----------------------------------------------------------------------
    // Integration: cross-type equality & round trips
    // -----------------------------------------------------------------------

    #[test]
    fn round_trip_hex_rgb_hex() {
        let original = "1e90ff";
        let hex = Color::hex(original);
        let rgb = hex.to_rgb();
        assert_eq!(rgb.to_hex().to_string(), format!("#{original}"));
    }

    #[test]
    fn round_trip_rgb_hex_rgb() {
        let (r, g, b) = (50, 168, 82);
        let rgb = Color::rgb(r, g, b);
        let hex = rgb.to_hex();
        let back = hex.to_rgb();
        assert_eq!(back.r, r);
        assert_eq!(back.g, g);
        assert_eq!(back.b, b);
    }

    #[test]
    fn hex_and_rgb_equivalence() {
        let hex = Color::hex("ff7f50");
        let rgb = Color::rgb(255, 127, 80);
        assert!(hex.is_equal(&rgb));
        assert!(rgb.is_equal(&hex));
    }

    // -----------------------------------------------------------------------
    // color! macro
    // -----------------------------------------------------------------------

}
