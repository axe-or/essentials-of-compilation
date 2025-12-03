mod lexer;

use lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Error {
    UnknownCodepoint,
    UnterminatedString,
    InvalidEscapeSequence,
    InvalidMultiLineString,
}

fn main() {
    let source = r#"
    let x = 69; <<>>;
    let name = "Hello\n\t\"World!\"";
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

        if tk == Token::EndOfFile {
            break;
        }

        println!("{:?}", tk);
    }
}

