fn main() {
    let mut s = String::from("Marcus");
    let len = get_length(&mut s);
    println!("{s} len is  {len}")
}

fn get_length(a: &String) -> usize {
    a.len()
}
