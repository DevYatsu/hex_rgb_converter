# hex_rgb_converter

Dead simple RGB ↔ hex color conversion.

```rust
let hex = Color::hex("ff7f50");
let rgb = Color::rgb(255, 127, 80);

// Convert between representations.
let back_to_hex = rgb.to_hex();
let back_to_rgb = hex.to_rgb();

// Compare across formats.
assert!(rgb.is_equal(&hex));
```

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
hex_rgb_converter = "1"
```

Then use in your code:

```rust
use hex_rgb_converter::{Color, RgbColor, HexColor, Colors};
```

## Examples

### Hex → RGB

```rust
let hex = Color::hex("787878");
let rgb: RgbColor = hex.to_rgb();
println!("{hex}");              // #787878
println!("{rgb}");              // (120,120,120)
```

### RGB → Hex

```rust
let rgb = Color::rgb(18, 239, 120);
let hex: HexColor = rgb.to_hex();
println!("{hex}");              // #12ef78
```

### Builder-style mutation

```rust
let mut rgb = Color::rgb(23, 2, 255);
rgb.set_blue(90)
   .set_green(90)
   .set_red(90)
   .set_color(Colors::All(120));
println!("{rgb}");              // (120,120,120)
```

### Named colours

```rust
Color::by_name("orange").print();   // #ffa500
Color::by_name("coral").print();    // #ff7f50
```

### Equality checks

```rust
let hex = Color::hex("fff");
let rgb = Color::rgb(0, 0, 255);

println!("{}", hex.is_equal(&rgb));             // false (blue ≠ white)
println!("{}", RgbColor::are_equal(&rgb, &rgb)); // true
println!("{}", HexColor::are_equal(&hex, &hex)); // true
```

### `color!` macro (optional)

The `color!` macro dispatches on argument count:

```rust
use hex_rgb_converter::color;

let a = color!("ff7f50");           // calls Color::hex
let b = color!(255, 127, 80);       // calls Color::rgb
```

This is the **only** macro provided — use `Color::hex()` and `Color::rgb()`
directly when you don't need the shorthand.

## License

MIT. See [LICENSE](./LICENSE).
