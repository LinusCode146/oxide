use std::error::Error;
use oxide::environment::Environment;
use oxide::evaluator::eval_program;
use oxide::lexer::Lexer;
use oxide::object::Object;
use oxide::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut l = Lexer::new(String::from("script.oxide"))?;
    l.convert_to_tokens();
    let mut parser = Parser::new(l);
    let program = parser.parse_program();
    parser.check_parser_errors();

    let mut env = Environment::new();
    let result = eval_program(&program, &mut env);
    if !matches!(result, Object::Null) {
        println!("{}", result);
    }

    Ok(())
}