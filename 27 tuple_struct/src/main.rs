fn main() {
    //TUPLE STRUCTS
    struct Color(i64, i64, i64);
    struct Punto(i64, i64, i64);

    let negro=Color(0, 0, 0);
    let p1=Punto(1,2,3);

    println!("Negro: {}, {}, {}", negro.0, negro.1, negro.2);
    println!("Punto: {}, {}, {}", p1.0, p1.1, p1.2);
}




