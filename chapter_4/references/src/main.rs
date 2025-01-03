fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");
    //println!("r2: {r2}");

    let reference_to_nothing = dangle();
}


fn dangle() -> &String {
    let s: String = String::from("hello");

    &s
}