fn main() {
   // slice type
   let mut s=String::from("Hola Mundo");
    println!("{}", primera_palabra(&s));
    s.clear();
    println!("Texto: {}", s);
}
fn primera_palabra(s: &String)->usize{
    let bytes=s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '
        {
            
            return i;
        }
    }
    s.len()
}



