#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle { p1, p2 } = rect;
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;

    (x1.abs() - x2.abs()).abs() * (y1.abs() - y2.abs()).abs()
}

fn square(point: Point, side: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = point;
    let new_point = Point {
        x: x1 + side,
        y: y1 + side,
    };
    Rectangle {
        p1: point,
        p2: new_point,
    }
}

fn main() {
    let name = "Prasanna";
    let age = 27;
    let prasanna = Person { name, age };

    println!("{:?}", prasanna);

    let point = Point { x: -13.0, y: 36.0 };

    println!("point coordinates {:?}", point);

    let new_point = Point { x: -1.0, y: 25.0 };

    let Point { x: my_x, y: my_y } = new_point;

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);

    println!(
        "Area of rectangle {:?} is {}",
        _rectangle,
        rect_area(&_rectangle)
    );
    let sqr = square(point, 4.0);

    println!(
        "Square from point is {:?} and its area is {}",
        sqr,
        rect_area(&sqr)
    );
}
