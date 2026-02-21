use std::error::Error;
use oxide::ast::Node;
use oxide::lexer::Lexer;
use oxide::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut l = Lexer::new(String::from("script.oxide"))?;
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();
    println!("{}", program.string());
    Ok(())
}


