fn main() {
    //ITERACIONES CON: LOOP
    let mut i=0;
    let suma = loop{
        if i==25{
            break i;
        }
        i+=1;
    };

    println!("Suma: {}", suma);
}
