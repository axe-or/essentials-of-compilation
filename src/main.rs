
pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Real(f64),
    Plus,
    Minus,

    EndOfFile,
}

fn is_part_of_identifier(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..'9' | '_' => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum Error {
    UnknownCodepoint,
    UnterminatedString,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            current: 0,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.current >= self.source.len(){
            return None;
        }
        self.current += 1;
        return Some(self.source[self.current - 1]);
    }

    pub fn peek(&self) -> Option<char> {
        if self.current >= self.source.len(){
            return None;
        }
        return Some(self.source[self.current]);
    }

    fn skip_whitespace(&mut self){
        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => break,
            };

            if !c.is_whitespace(){
                self.current -= 1;
                break;
            }
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.current;

        loop {
            let c = match self.advance(){
                Some(c) => c,
                None => return Token::EndOfFile,
            };

            if !is_part_of_identifier(c){
                self.current -= 1;
                break;
            }
        }

        assert!(self.current > start, "Invalid identifier length");

        let lexeme: String = (&self.source[start..self.current]).iter().collect();

        return Token::Identifier(lexeme);
    }

    fn scan_integer(&mut self) -> Option<Token>{
        todo!();
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::EndOfFile),
        };

        if is_part_of_identifier(c){
            return Ok(self.scan_identifier());

        }

        return Err(Error::UnknownCodepoint);
    }
}

fn main() {
    let source = r"
    let x = 69
    ";

    let mut lex = Lexer::new(source);

    loop {
        let tk = match lex.next() {
            Ok(tk) => tk,
            Err(e) => {
                println!("{:?}", e);
                break;
            },
        };

        println!("{:?}", tk);
    }

}
