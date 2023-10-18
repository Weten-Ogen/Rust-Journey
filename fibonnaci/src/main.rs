fn main() {
    for i in 1..9 {
        println!("Finbonacci of {i} => {}", call_fibonnaci(i));
    }
}

fn call_fibonnaci(num:i32) -> i32 {
    if num <= 0 {
         return 0;
    }  else if num == 1 {
        return 1;
    } else {
        return call_fibonnaci(num - 1) + call_fibonnaci(num - 2)

    }
}