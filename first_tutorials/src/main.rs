fn main() {
    println!("{}",fizz_buz("f"));
}

fn fizz_buz(s:&str)-> &str {
    match s {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "***Nothing",
    }
}