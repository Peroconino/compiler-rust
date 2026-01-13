use std::fs;
use std::io;

use lexer::Lexer;

mod lexer;
mod token;

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path)?;

    let _lexer = Lexer::new(contents);

    Ok(())
}
