fn main(){
    //VALORES DE RETORNO Y ALCANCE
    let s1=dar_valor();
    let s2=String::from("Esta es s2");
    let s3=tomar_valor_y_devolverlo(s2);
    println!("s1: {}", s1);
    //println!("s2: {}", s2);
    println!("s3: {}", s3);
}
fn dar_valor()->String
{
    let s1_interna=String::from("Esta es s1");
    s1_interna
}
fn tomar_valor_y_devolverlo(variable_interna: String)->String{
    variable_interna
}