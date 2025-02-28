mod tokenizer;
mod parser;

use std::{fs::File, io::Read};

fn main() {
    let mut file = match File::open("main.c") {
        Ok(res) => res,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let tokens = tokenizer::tokenize(file_content.as_str());

    println!("{:?}", tokens);
}
