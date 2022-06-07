struct Rectangulo{
    alto:u32,
    ancho:u32 
}
impl Rectangulo{
    fn cuadrado(lado:u32)->Rectangulo{
        Rectangulo{
          alto:lado,
          ancho:lado  
        }
    }
    fn area(&self)->u32{
        self.alto*self.ancho
    }
}
fn main() {
    //FUNCIONES ASOCIADAS A ESTRUCTURAS
    let c1=Rectangulo::cuadrado(4);
    
    println!("Área: {}", c1.area());
}
