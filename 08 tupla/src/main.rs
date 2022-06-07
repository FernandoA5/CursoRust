fn main() {
    //TUPLAS
    let tupla: (i32, f64, char, bool)=(24, 3.14159265, 'a', false);

    let (_a, b, _c, _d)=tupla;

    println!("Tupla: {}", b);

}
