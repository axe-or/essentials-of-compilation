
pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

pub enum Token {
    Identifier(String),
    Integer(i64),
    Real(f64),
    Plus,
    Minus,
}

fn is_part_of_identifier(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..'9' | '_' => true,
        _ => false,
    }
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

    fn scan_identifier(&mut self) -> Option<Token> {
        let start = self.current;

        loop {
            let c = match self.advance(){
                Some(c) => c,
                None => break,
            };

            if !is_part_of_identifier(c){
                self.current -= 1;
                break;
            }
        }

        if start == self.current {
            return None;
        }

        let lexeme: String = (&self.source[start..self.current]).iter().collect();
        println!("'{}'", lexeme);

        return Some(Token::Identifier(lexeme));
    }

    fn scan_integer(&mut self) -> Option<Token>{
        todo!();
    }
}

fn main() {
    let source = r"
    let x = 69
    ";

    let mut lex = Lexer::new(source);

    lex.skip_whitespace();
    lex.scan_identifier();

    println!("Hello, world!");
}
