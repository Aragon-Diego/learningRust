use std::io;

fn main() {
    println!("Entra el número de index de número de febonacci");
    let mut nth = String::new();
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");
    let nth:i32 = nth.trim().parse().expect("Escribe un número válido");
    if nth <= 0 {
        panic!("Escribe un número válido");
    }
    let mut step = 0;
    let mut last = 0;
    let mut current = 0;
    let mut first = true;
    println!("<----- Secuenciad ----->");
    while step <= nth {
        if first && last == 1 {
            last = 0;
            current = 1;
            first = false;
        }
        let next_fibonacci = last + current;
        last = current;
        current = next_fibonacci;
        if current == 0 {
            current += 1;
        }
        step += 1;
        println!("{next_fibonacci}");
    }
}
