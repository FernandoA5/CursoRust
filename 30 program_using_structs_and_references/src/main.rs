struct Rectangulo{
    alto: u32,
    ancho: u32
}
fn main() {
   //Program Using Functions, Structs & References
    let rec=Rectangulo{
        alto:25,
        ancho:45
    };
    println!("Área: {}", area(&rec));
}
fn area(rec_local:&Rectangulo) ->u32{
    rec_local.alto*rec_local.ancho
}
