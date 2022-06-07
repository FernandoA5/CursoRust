fn main() {
    //BORROWING
    let mut variable = String::from("Hola");
    println!("Variable: {}", variable);
    cambiar(&mut variable);
    println!("Variable: {}", variable);
    
    let r1= &variable;
    let r2= &variable;
    let r3= &mut variable;
    println!("R1: {} || R2: {} || R3: {}", r1, r2, r3);
}
fn cambiar(texto_local: &mut String){
    texto_local.push_str(" Mundo");
}
