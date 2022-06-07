fn main(){
    //PROGRAM USING TUPLES
    let r =(25, 45);
    println!("Área: {}", area(r));
}
fn area(d: (u32, u32))->u32{
    d.0 * d.1
}




