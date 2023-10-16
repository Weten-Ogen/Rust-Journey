fn main() {
    let s1 = give_ownership();
    let s2 = String::from("Hello");
    let s3  = takes_and_gives_back(s2);
    
}

fn give_ownership() -> String {
    let ss  = String::from("world");
    ss
}
fn takes_and_gives_back(a:String) -> String {
    a
}