fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // arrays
    let a = [1, 2, 3, 4, 5];
}