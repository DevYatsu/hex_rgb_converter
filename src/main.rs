use hex_rgb_converter::{RgbColor, HexColor, Color, Colors};

#[macro_use] mod macros;

fn main() {
    let my_color: HexColor = color!("787878");
    let snd_color: RgbColor = my_color.to_rgb();
    println!("{} {}", my_color, snd_color);

    let test_color: HexColor = Color::hex("#12ef78").to_rgb().to_hex();
    test_color.print();

    let mut rgb_col: RgbColor = color!(23, 2, 255);
    rgb_col
        .set_blue(90)
        .set_green(90)
        .set_red(90)
        .set_color(Colors::All(120));
    println!("{} {}", rgb_col, rgb_col.to_hex());

    let mut rgb_2: RgbColor = Color::rgb(2, 4, 5);
    rgb_2.set_color(Colors::Blue(120));
    rgb_2.print();

    println!("{}", rgb_2.is_equal(&my_color));

    rgb_2.to_hex().print();

    println!("{}", RgbColor::are_equal(&rgb_col, &rgb_2));

    Color::by_name("orange").print();
}

