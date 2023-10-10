use core::panic;
use::std::fs::File;
use::std::io::ErrorKind;
fn main() {
    let _greeting_file_result = File::open("hello.txt");
    let greeting_file  = match _greeting_file_result {
        Ok(File) => File,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}",other_error)
            }
        }
    };

}
