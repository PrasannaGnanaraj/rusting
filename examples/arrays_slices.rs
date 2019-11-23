use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice {}", slice[0]);
    println!("the slice has {} elenebts", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of the array {}", xs[0]);
    println!("second element of the array {}", xs[1]);

    println!("array size {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow whole array as slice");

    analyze_slice(&xs);

    println!("borrow the section of array as slice");
    analyze_slice(&ys[1..4]);

    // println!("{}", xs[5]);
}
