use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let x = "let x = \"this is a string.\" == \"this is another string.\";";

    let tokens = Lexer::new(x).tokenize()?;

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.get_statements()?);

    // println!("{:?}", parser.get_statements());

    // Interpreter::repl()?;

    Ok(())
}
