mod lexer;

use lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum Error {
    UnknownCodepoint,
    UnterminatedString,
    InvalidEscapeSequence,
    InvalidOperator,
    InvalidMultiLineString,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn from_source(source: &str) -> Parser {
        Parser::from_lexer(Lexer::new(source))
    }

    pub fn from_lexer(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        match self.lexer.next() {
            Ok(tk) => Some(tk),
            Err(e) => {
                println!("Error: {:?} at {}", e, self.lexer.current);
                None
            },
        }
    }

    fn peek(&mut self) -> Option<Token> {
        match self.lexer.get_token() {
            Ok(tk) => Some(tk),
            Err(e) => {
                println!("Error: {:?} at {}", e, self.lexer.current);
                None
            },
        }
    }
}

fn main() {
    let source = r#"
    69 + 7 / (5 + 420)
    "#.trim();


    let mut lex = Lexer::new(source);
    println!("{}", source.trim());

    loop {
        let tk = match lex.next() {
            Ok(v) => v,
            Err(e) => {
                println!("[{}] Error {:?}", lex.current, e);
                break;
            },
        };

        if tk.kind == TokenKind::EndOfFile {
            break;
        }

        println!("{:?}", tk);
    }
}

