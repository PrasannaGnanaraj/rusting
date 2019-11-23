#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};

    use crate::Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("The rich get richer"),
        Poor => println!("The poor get poorer . . ."),
    }

    match work {
        Civilian => println!("Civilians work"),
        Soldier => println!("Soldiers fight"),
    }
}
