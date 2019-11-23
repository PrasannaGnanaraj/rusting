use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} ) \n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, 0.1f32, '1', true);

    println!("long tuple first value is {}", long_tuple.0);

    let tuple_of_tuples = ((1, 2, 3), (4, -1), 1);

    println!("tuple of tuples {:?}", tuple_of_tuples);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple {:?}", too_long_tuple);

    let pair = (1, true);
    println!("the pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple {:?}", (4,));

    println!("just an integer {:?}", (3));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{}", matrix);

    println!("transposed");

    println!("{}", transpose(matrix));
}
