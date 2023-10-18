#![allow(unused)]
fn adder(a: i32, b:i32) -> i32{
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4,adder(2,2));
    }
}
