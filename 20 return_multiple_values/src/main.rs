fn main() {
    //Retornar valores con tuplas
    let texto=String::from("HOLA");
    let (t1, long) = calcular_longitud(texto);

    println!("t1: {} || long: {}", t1, long);
}
fn calcular_longitud(texto_local: String)->(String, usize){
    let longitud=texto_local.len();
    (texto_local, longitud)
}





