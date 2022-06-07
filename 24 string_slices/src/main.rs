fn main() {
    //STRING SLICES
    let s = String::from("Hola Mundo");
    let hola = &s[..4];
    let mundo = &s[5..];
    let hola_mundo = &s[..];
    println!("Hola: {}", hola);
    println!("Mundo: {}", mundo);
    println!("Hola Mundo: {}", hola_mundo);
}






