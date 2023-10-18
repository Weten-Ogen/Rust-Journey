#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(),string2);
    println!("the longest string is {}",result)
}

fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}