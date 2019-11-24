fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("hakuna matata");
        } else if n % 3 == 0 {
            println!("hakuna");
        } else if n % 5 == 0 {
            println!("matata");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
