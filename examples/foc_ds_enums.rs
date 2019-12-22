#![allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMV(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it ?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {} Green: {} Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {} Saturation: {} Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {} Saturation: {} lightness: {}", h, s, l),
        Color::CMV(c, m, v) => println!("Cyan: {} Magenta: {} Violet: {}", c, m, v),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {} Magenta: {} Violet: {} Key (black): {}",
            c, m, y, k
        ),
    }
}
