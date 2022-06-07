struct Usuario{
    id: i64,
    nombre: String
}
fn main() {
    //ESTRUCTURAS
    let us1=crear_estructura(2, String::from("Nombre"));
    println!("Id: {}",us1.id);
    println!("Nombre: {}",us1.nombre);
}

fn crear_estructura(id_local: i64, nombre_local: String)->Usuario{
    Usuario{
        id: id_local,
        nombre: nombre_local
    }
}
