struct Usuario{
    name: String,
    correo: String,
    id: i64,
    activo: bool
}

fn main(){
    //Sintaxis de actualización de estructura
    let us1= Usuario{
        name:String::from("Fernando"),
        correo:String::from("correo@correo.com"),
        id: 1,
        activo: true
    };
    let us2= Usuario{
        name:String::from("Otro Nombre"),
        correo:String::from("otro@correo.com"),
        ..us1
    };
    println!("Nombre: {}", us1.name);
    println!("Correo: {}", us1.correo);
    println!("Id: {}", us1.id);
    println!("Activo: {}", us1.activo);
    println!("Nombre: {}", us2.name);
    println!("Correo: {}", us2.correo);
    println!("Id: {}", us2.id);
    println!("Activo: {}", us2.activo);
    
}