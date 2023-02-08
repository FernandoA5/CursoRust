enum Billete{
    Benito,
    Morelos,
    Neza,
    SorJuana
}
fn main(){
    let mi_billete = Billete::Benito;
    println!("Valor: {}", valor_billete(mi_billete));
}
fn valor_billete(mi_billete: Billete) -> u16
{   
    match mi_billete {
        Billete::Benito => {
            println!("Este billete tiene la cara de Benito Juarez");
            20
        },
        Billete::Morelos => 50,
        Billete::Neza => 100,
        Billete::SorJuana => 200
    }
}
