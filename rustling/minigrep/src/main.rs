#![allow(unused)]
use std::env;
use std::fs;

fn main() { 
    let args:Vec<String> = env::args().collect();
    let (query,filename) = parse_config(&args);

    let content = get_file(&filename);

    .expect("Should have read the file");
    println!("with text:\n{content}");
    println!("You searched for {} in {} file",query,filename)

}

struct Config {
    query : String,
    filename: String,
}

fn get_args(args: &[String]) -> Config  {
    let query = args[1].clone();
    let filename = vargs[2].clone();
    Config { query, filename}
}

fn get_file(filename: &str) -> Result<String,E>  {
    fs::read_to_string(filename)
}