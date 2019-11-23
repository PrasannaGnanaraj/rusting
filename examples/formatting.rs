use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn hex_lookup(number: u8) -> String {
    // let num = number as i32;
    // match num {
    //     0..=9 => format!("{}", num),
    //     10 => String::from("A"),
    //     11 => String::from("B"),
    //     12 => String::from("C"),
    //     13 => String::from("D"),
    //     14 => String::from("E"),
    //     15 => String::from("F"),
    //     _ => format!("Err {}", num),
    // }

    format!("{:02X}",number)
}

fn rgb_hex(inp: u8) -> String {
    // let divider = inp as f32 / 16.0;
    // let remaining = (divider - (inp / 16) as f32) * 16.0;
    format!("{}", hex_lookup(inp))
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({},{},{}) 0x{}{}{}",
            self.red,
            self.green,
            self.blue,
            rgb_hex(self.red),
            rgb_hex(self.green),
            rgb_hex(self.blue)
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
