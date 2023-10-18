fn main() {
    let  b = String::from("Marcus");
    first_world(&b);
    let s = String::from("hello world");
    second_word(&s);
}

fn first_world(s: &String) -> usize {
    let bytes= s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item  == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s: &String) {
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}");
    println!("{world}");
} 
