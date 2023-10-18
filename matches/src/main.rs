#[derive(Debug)]
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter
}
fn main() {
    let money  = value_in_cents(Coins::Penny);
    println!("{money}")
}

fn value_in_cents(coin:Coins) -> u8 {
     match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25
     }
}
