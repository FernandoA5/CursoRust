#[derive(Debug)]
enum IPAddrssKind{
    IpV4(u8, u8, u8, u8),
    IpV6(String)
}
fn main() {
    let casa = IPAddrssKind::IpV4(127, 0, 0, 1);
}
