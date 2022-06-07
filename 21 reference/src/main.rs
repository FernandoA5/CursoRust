//REFERENCIAS y PRESTAMOS
fn main(){
    let texto=String::from("HOLA");
    println!("Texto {}", texto);
}
fn modificar(texto_local: &String){
    texto_local.push_str(" MUNDO");
}






