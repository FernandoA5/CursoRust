struct Rectangulo{
    alto:u32,
    ancho: u32
}
impl Rectangulo {
    fn area(&self)->u32{
        self.alto*self.ancho
    }
}

fn main(){
    //definir_metodos de estructuras
    let r1= Rectangulo{
        alto:24,
        ancho:45
    };
    println!("Área: {}", r1.area())
}
