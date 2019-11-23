fn main() {
    println!("{} days", 32);
    println!("{0}, this is {1}. {1} this is {0}", "prasanna", "leanne");

    println!(
        "{subject} {verb} {object}",
        object = "suitcase",
        subject = "prasanna",
        verb = "broke"
    );

    println!("{} of {:b} people know binary, the other half does't", 1, 2);

    println!("{number:>width$}", number = 2, width = 6);

    println!("{number:>0width$}", number = 2, width = 6);

    println!("My name is {0} , {1} {0}", "bond", "james");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:?} won't print . . .", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
