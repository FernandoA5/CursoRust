struct Rectangulo{
    alto:u32,
    ancho:u32
}

impl Rectangulo{
    fn area(&self)->u32
    {
        self.alto*self.ancho
    }
}
impl Rectangulo{
    fn cuadrado(lado:u32)->Rectangulo{
        Rectangulo{
            alto:lado,
            ancho:lado
        }
    }
}

fn main() {
    //Multiples Bloques de Implementación
    let rec1 = Rectangulo::cuadrado(5);
    let area=rec1.area();
    println!("Área: {}", area);
}
