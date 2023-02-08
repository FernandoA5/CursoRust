#[derive(Debug)]
enum Year{
    Year1964,
    Year1994,
    Year2012

}
enum Billete{
    Benito,
    Morelos,
    Neza(Year),
    SorJuana
}
fn main(){
    let mi_billete = Billete::Benito;
    println!("Valor: {}", valor_billete(mi_billete));

    let mi_nuevo_billete = Billete::Neza(Year::Year1964);
    println!("Valor: {}", valor_billete(mi_nuevo_billete));
}
fn valor_billete(mi_billete: Billete) -> u16
{   
    match mi_billete {
        Billete::Benito => {
            println!("Este billete tiene la cara de Benito Juarez");
            20
        },
        Billete::Morelos => 50,
        Billete::Neza(emision) =>{
            println!("Fecha de emisión: {:?}", emision);
            100
        }
        Billete::SorJuana => 200
    }
}