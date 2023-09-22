fn main() {
    println!("Hello world");
    another_func();
    middle_func(10);
}

fn another_func() {
    println!("Hello another")
}

fn middle_func(x:u32 ) {
    println!("The number is {x}")
}