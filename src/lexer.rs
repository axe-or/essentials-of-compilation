use crate::Error;

pub struct Lexer {
    pub source: Vec<char>,
    pub current: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
}

impl Token {
    pub fn new(kind: TokenKind, offset: usize) -> Token {
        Token {
            kind: kind,
            offset: offset,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    Identifier(String),
    String(String),
    Integer(i64),
    Constant,

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

    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,

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

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            current: 0,
        }
    }

    fn make_lexeme(&self, start: usize, end: usize) -> String {
        (&self.source[start..end]).iter().collect()
    }

    fn advance(&mut self) -> Option<char> {
        if self.current >= self.source.len(){
            return None;
        }
        self.current += 1;
        return Some(self.source[self.current - 1]);
    }

    fn peek(&self) -> Option<char> {
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
        let kind = match as_keyword(&lexeme){
            Some(kw) => kw,
            None => TokenKind::Identifier(lexeme),
        };

        return Token { kind, offset: start };
    }

    fn scan_string(&mut self) -> Result<Token, Error> {
        assert!(self.advance() == Some('"'), "Invalid lexer position");

        let mut buf = String::new();
        let offset = self.current;

        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => return Err(Error::UnterminatedString),
            };

            if c == '\\' {
                let next_char = match self.advance() {
                    Some(c) => c,
                    None => return Err(Error::UnterminatedString),
                };

                let escaped = match escape_sequence(next_char) {
                    Some(c) => c,
                    None => return Err(Error::InvalidEscapeSequence),
                };

                buf.push(escaped);
            }
            else if c == '\n' || c == '\r' {
                return Err(Error::InvalidMultiLineString);
            }
            else if c == '"' {
                break;
            }
            else {
                buf.push(c);
            }
        }

        let kind = TokenKind::String(buf);
        return Ok(Token::new(kind, offset));
    }

    fn scan_decimal_integer(&mut self) -> Result<Token, Error>{
        let offset = self.current;

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

        let lexeme = self.make_lexeme(offset, self.current);
        let num = lexeme.parse::<i64>().expect("Invalid integer");
        let kind = TokenKind::Integer(num);

        return Ok(Token::new(kind, offset));
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

    pub fn get_token(&mut self) -> Result<Token, Error> {
        let restore = self.current;
        let res = self.next();
        self.current = restore;
        return res;
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        use TokenKind as T;

        self.skip_whitespace();

        let offset = self.current;

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::new(T::EndOfFile, offset)),
        };

        if c.is_numeric(){
            return self.scan_decimal_integer();
        }

        if is_part_of_identifier(c){
            return Ok(self.scan_identifier());
        }

        if c == '"' {
            return self.scan_string();
        }

        _ = self.advance();

        let kind = match c {
            '.' => Ok(T::Dot),
            ':' => Ok(T::Colon),
            ',' => Ok(T::Comma),
            ';' => Ok(T::Semicolon),

            '(' => Ok(T::ParenOpen),
            ')' => Ok(T::ParenClose),
            '[' => Ok(T::SquareOpen),
            ']' => Ok(T::SquareClose),
            '{' => Ok(T::CurlyOpen),
            '}' => Ok(T::CurlyClose),

            '+' => Ok(T::Plus),
            '-' => Ok(T::Minus),
            '*' => Ok(T::Star),
            '/' => Ok(T::Slash),
            '%' => Ok(T::Modulo),
            '~' => Ok(T::Tilde),
            '|' => Ok(T::Or),
            '&' => Ok(T::And),

            '!' => if self.match_advance('='){
                Ok(T::NotEqual)
            } else {
                Err(Error::InvalidOperator)
            }
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
        }?;

        return Ok(Token::new(kind, offset));
    }
}

fn as_keyword(s: &str) -> Option<TokenKind> {
    use TokenKind as T;

    static KEYWORDS: [(&'static str, TokenKind); 4] = [
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

