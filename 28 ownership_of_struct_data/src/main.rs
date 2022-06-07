struct Superficie{
    alto:u32,
    ancho: u32
}
fn main() {
    //Estructuras Ejemplo Practico
    let superficie= Superficie{
        alto:25,
        ancho:45
    };
    println!("AREA: {}", area(superficie));
}
fn area(superficie_local: Superficie) -> u32{
    superficie_local.alto*superficie_local.ancho
}