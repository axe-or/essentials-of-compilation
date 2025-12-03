pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Real(f64),

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    ShiftRight,
    ShiftLeft,
    Tilde,
    And,
    Or,

    Equal,
    NotEqual,
    Gt,
    GtEq,
    Lt,
    LtEq,

    Assign,
    Dot,
    Semicolon,
    Colon,
    Comma,

    Fn,
    If,
    Else,
    Let,

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

fn escape_sequence(c: char) -> Option<char> {
    match c {
        'r' => Some('\r'),
        'n' => Some('\n'),
        't' => Some('\t'),
        'e' => Some('\x1B'),
        '"' => Some('"'),
        '\'' => Some('\''),
        '\\' => Some('\\'),
        _ => None,
    }
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            current: 0,
        }
    }

    pub fn make_lexeme(&self, start: usize, end: usize) -> String {
        (&self.source[start..end]).iter().collect()
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
                None => break,
            };

            if !is_part_of_identifier(c){
                self.current -= 1;
                break;
            }
        }

        assert!(self.current > start, "Invalid identifier length");

        let lexeme: String = self.make_lexeme(start, self.current);
        if let Some(tok) = as_keyword(&lexeme){
            return tok;
        }

        return Token::Identifier(lexeme);
    }

    fn scan_string(&mut self) -> Result<Token, Error> {
        todo!();
    }

    fn scan_decimal_integer(&mut self) -> Result<Token, Error>{
        let start = self.current;

        loop {
            let c = match self.advance(){
                Some(c) => c,
                None => break,
            };

            if !c.is_numeric() && c != '_' {
                self.current -= 1;
                break;
            }
        }

        let lexeme = self.make_lexeme(start, self.current);
        println!("'{}'", lexeme);
        let num = lexeme.parse::<i64>().expect("Invalid integer");

        return Ok(Token::Integer(num));
    }

    fn match_advance(&mut self, target: char) -> bool {
        if let Some(c) = self.peek() {
            if c == target {
                self.current += 1;
            }
            return c == target;
        }
        return false;
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        use Token as T;

        self.skip_whitespace();

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::EndOfFile),
        };

        if c.is_numeric(){
            return self.scan_decimal_integer();
        }

        if is_part_of_identifier(c){
            return Ok(self.scan_identifier());
        }

        _ = self.advance();

        let tk = match c {
            '.' => Ok(T::Dot),
            ':' => Ok(T::Colon),
            ',' => Ok(T::Comma),
            ';' => Ok(T::Semicolon),

            '+' => Ok(T::Plus),
            '-' => Ok(T::Minus),
            '*' => Ok(T::Minus),
            '/' => Ok(T::Slash),
            '%' => Ok(T::Modulo),
            '~' => Ok(T::Tilde),
            '|' => Ok(T::Or),
            '&' => Ok(T::And),
            '=' => if self.match_advance('='){
                Ok(T::Equal)
            } else {
                Ok(T::Assign)
            }

            '>' => if self.match_advance('='){
                Ok(T::GtEq)
            } else if self.match_advance('>') {
                Ok(T::ShiftRight)
            } else {
                Ok(T::Gt)
            },

            '<' => if self.match_advance('='){
                Ok(T::LtEq)
            } else if self.match_advance('<') {
                Ok(T::ShiftLeft)
            } else {
                Ok(T::Lt)
            },

            _ => Err(Error::UnknownCodepoint),
        };


        return tk;
    }
}

fn as_keyword(s: &str) -> Option<Token> {
    use Token as T;

    static KEYWORDS: [(&'static str, Token); 4] = [
        ("if", T::If),
        ("else", T::Else),
        ("let", T::Let),
        ("fn", T::Fn),
    ];

    for (key, token) in &KEYWORDS {
        if s == *key {
            return Some(token.clone());
        }
    }

    return None;
}

fn main() {
    let source = r"
    let x = 69; <<>>
    ".trim();

    let mut lex = Lexer::new(source);
    println!("{}", source.trim());

    loop {
        let tk = match lex.next() {
            Ok(tk) => tk,
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
