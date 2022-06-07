struct Rectangulo{
    alto:u32,
    ancho:u32
}
impl Rectangulo{
    fn can_hold(&self, otro: &Rectangulo) -> bool{
        self.alto > otro.alto && self.ancho > otro.ancho
    }
}
//MÉTODOS CON MULTIPLES PARAMETROS
fn main() {
    let rec1 = Rectangulo{
        alto:10,
        ancho:20
    };
    let rec2 = Rectangulo{
        alto:5,
        ancho:3
    };
    let rec3 = Rectangulo{
        alto:90,
        ancho: 50
    };
    println!("can hold: {}", rec1.can_hold(&rec3));
}
