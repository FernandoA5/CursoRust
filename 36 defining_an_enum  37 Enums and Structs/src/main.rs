//Enum Values
#[derive(Debug)]
enum IpAddrsKind{
    V4,
    V6
}
struct IpAddrs{
    knid: IpAddrsKind,
    address: String
}
fn main() {
    let ip_home = IpAddrs{
        kind:IpAddrsKind::V4,
        address: Stirng::from("192.168.1.1")
    };
    
    println!("{:?}", home);
    println!("{:?}", Some(5));
}
