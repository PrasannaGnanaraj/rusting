fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("hakuna matata");
        } else if n % 3 == 0 {
            println!("hakuna");
        } else if n % 5 == 0 {
            println!("matata");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "shippis",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
