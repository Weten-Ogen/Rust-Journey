#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address:String::from("127.0.0.1")
    };
    let some_char:Option<char> = Some('e');
    let some_numb:Option<i32>   = Some(5);

    println!("{:?} ",some_char)
   
}

