use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:b}, y: {:b}", self.x.to_bits(), self.y.to_bits())
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} + {:.1}i", self.real, self.imag)
    }
}

fn main() {
    let structure = Structure(14);

    println!("Compare Structures:");
    println!("Display: {}", structure);
    println!("Debug: {:?} \n", structure);

    let minmax = MinMax(0, 14);

    println!("Compare MinMaxes:");
    println!("Display: {}", minmax);
    println!("Debug: {:?} \n", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small} \n",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 4.5 };

    println!("Compare Points:");
    println!("Display: {}", point);
    println!("Display: {:?}", point);
    println!("What does Point2D look like in binary: {:b}? \n", point);

    let complex = Complex {
        real: 3.4,
        imag: 7.3,
    };
    println!("Compare Comples:");
    println!("Display: {}", complex);
    println!("Display: {:?}", complex);
}
