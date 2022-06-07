#[derive(Debug)]
struct Rectangulo{
    alto:u32,
    ancho:u32
}

fn main() {
    let r1= Rectangulo{
        alto:25,
        ancho:45
    };
    println!("R1: {:?}", r1);
}


