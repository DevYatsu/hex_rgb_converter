/// Create a colour from a hex string or RGB components.
///
/// With one argument it behaves like [`Color::hex`]; with three it behaves
/// like [`Color::rgb`].
///
/// ```
/// use hex_rgb_converter::Color;
///
/// let a = color!("ff7f50");
/// assert_eq!(a.to_string(), "#ff7f50");
///
/// let b = color!(255, 127, 80);
/// assert_eq!(b.to_string(), "(255,127,80)");
/// ```
#[macro_export] macro_rules! color {
    ($color: expr) => {
        Color::hex($color)
    };
    ($r: expr, $g: expr, $b: expr) => {
        Color::rgb($r, $g, $b)
    };
}
