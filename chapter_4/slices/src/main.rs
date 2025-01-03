// fn main() {
//     let s = String::from("LOS CINCO GODS DEL MELEE SON MANGO, ARMADA, M2k, HBOX y PPMD");
//     let first_word = get_first_word(s);
//     println!("La primera palabra es: \"{first_word}\"")
// }

// fn get_first_word(s: String) -> String {
//     let chunks = s.split_whitespace()
//         .next()
//         .unwrap_or("");
//     return String::from(chunks);
// }

fn main() {
    let s = String::from("LOS CINCO GODS DEL MELEE SON MANGO, ARMADA, M2k, HBOX y PPMD");
    //let first_word = get_first_word(&s);
    let first_word = first_word(&s);
    println!("La primera palabra es: \"{first_word}\"")
}

fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}