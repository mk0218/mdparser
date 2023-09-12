mod document;
mod parser;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::parser::MDParser as Parser;


fn main() -> io::Result<()> {
    let path = "./sample.md";
    // let path = "./README.md";

    let lines = match read_lines(path) {
        Ok(lines) => lines,
        Err(why) => panic!("Couldn't open {}: {}", path, why),
    };

    let document = Parser::parse(lines).unwrap();
    document.print();

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
