fn main() {
    //OWNERSHIP Y LAS FUNCIONES
    let x = 24;
    numero(x);
    println!("Número despues de la función: {}", x);
}
fn numero(numero: i32)
{
    println!("Número dentro de la función: {}", numero);
}


