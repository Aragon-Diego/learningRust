fn main() {
    let s: &str = " world";
    let mut s0 = String::from("hello");
    s0.push_str(s);
    println!("el string creado es el siguiente: \"{s0}\"");

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");
}
