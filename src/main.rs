use std::error::Error;
use oxide::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::new(String::from("script.coral"))?;
    lexer.print_file();
    lexer.convert_to_tokens();
    lexer.print_tokens();
    Ok(())
}


