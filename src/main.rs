use crate::lexer::Lexer;
use std::{fs, io};

mod lexer;
mod token;
fn main() -> io::Result<()> {
    let source = fs::read_to_string("src/codigo.test")?;
    let mut lexer = Lexer::new(source.to_string());
    println!("Código:\n{}", source);
    
    loop {
        let token = lexer.siguiente_token();
        println!("{:?}", token);
        if token.token_type == token::TokenType::EOF {
            break;
        }
    }
    Ok(())
}

