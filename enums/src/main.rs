#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let web_home = IpAddrKind::V4(127, 0, 0, 1);
    let web_loopback = IpAddrKind::V6(String::from("::1"));
    
    println!("Web home: {:?}", web_home);
    println!("Web loopback: {:?}", web_loopback);
}
