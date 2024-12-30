//Formula:
//  (°F) - 32) * 5/9.
use std::io;
fn main() {
    println!("Porfavor escribe la temperatura");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp:f32 = temp.trim().parse().expect("Escribe un número válido");
    println!("Tu temperatura en Fº es {temp}");
    let temp:f32 = (temp - 32.0) * (5.0 / 9.0);
    println!("Tu temperatura en Cº es {temp}");

}
