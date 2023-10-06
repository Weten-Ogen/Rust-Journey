use std::vec;


fn main() {
    let v  = vec![1,2,3,4,5];
    let third:&i32 =&v[2];
    println!("the third element is {third}");

    let third:Option<&i32> = v.get(4);
    match third {
        Some(third) => println!("The  third element is {third}"),
        None => println!("There is no third element"),
    };
    let mut m = vec![1,2,3,4,5,6];
    for i in &mut m {
        *i += 50;
    };
    let n = &m[3];
    println!("this is {}",n);
    let s = String::from("internal strife");
    for c in s.bytes() {
        println!("{c}")
    }

}
