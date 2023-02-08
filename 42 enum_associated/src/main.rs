#[derive(Debug)]
enum Year{
    Year1964,
    Year1994,
    Year2012
}
enum Billetes{
    Benito,
    Morelos,
    Neza(Year),
    SorJuana
}

fn main(){
    let mi_billete = Billetes::Benito;
    println!("Valor: {}", valor_billete(mi_billete));
    let mi_nuevo_billete_ligeramente_devaluado = Billetes::Neza(Year::Year1964);
    println!("Valor: {}", valor_billete(mi_nuevo_billete_ligeramente_devaluado));
}
fn valor_billete(mi_billete: Billetes) -> u16{
    match mi_billete{
        Billetes::Benito =>{
            println!("Este billete tiene la cara de Benito Juarez");
            20
        },
        Billetes::Morelos => 50,
        Billetes::Neza(emision) => {
            println!("Este Billete tiene una fecha de emisión de: {:?}", emision);
            100
        },
        Billetes::SorJuana => 200
    }
}