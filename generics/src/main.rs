
fn largest_T <T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest_T(&number_list);
    println!("The largest number is {}",result);

    let  char_list= vec!['y','m','a','q'];

    let result = largest_T(&char_list);
    println!("The largest char is {}",result);    
}
