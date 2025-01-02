fn main() {
    let arr = [("primer", "un perro"), ("segundo", "dos gallinas"), ("tercer", "tres carros"), ("cuarto", "cuatro vacas"), ("quinto", "cinco nintendo switch"), ("sexto", "seis camisas polo"), ("septimo", "siete perfumes") ,
    ("octavo", "ocho libros"), ("noveno", "nueve plumas"), ("decimo", "diez comics"), ("onceavo", "once macs pro m1"), ("doceavo", "doce pistaches")];
    for i in 0..arr.len() {
        let (palabra, compra) = arr[i];
        println!("El {palabra} día de Navidad, mi amor me mandó");
        let punctuation = if i == 0 {"."} else {", "};
        print!("{compra}{punctuation}");
        for j in 0..i {
            let index_to_eval = i - j - 1;
            let (_, temp_compra) = arr[index_to_eval];
            let temp_punctuation = if index_to_eval == 0 {"."} else {", "};
            print!("{temp_compra}{temp_punctuation}");
        }
        println!("");
    }
}
