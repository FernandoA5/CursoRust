fn main() {
    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six: {:?}", six);
    println!("None: {:?}", none);
    println!("Six: {}", get_number(six));
    println!("None: {}", get_number(none));
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
fn get_number(number: Option<i32>) -> i32 {
    match number{
        None => 0,
        Some(i) => i
    }
}
