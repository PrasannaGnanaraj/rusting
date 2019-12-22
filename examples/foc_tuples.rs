fn main() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, -2) => println!("x is {:?} and last is -2", x),
        (0, y) => println!("first is 0 and y is {:?}", y),
        _ => println!("It does'nt matter what they are"),
    }
}
