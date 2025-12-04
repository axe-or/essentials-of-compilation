mod lexer;

use lexer::{Lexer, Token, TokenKind};
use std::num::NonZeroU32;
use std::mem::size_of;

macro_rules! const_assert {
    ($x:expr, $msg:expr) => {
        const _: () = ::core::assert!($x, $msg);
    };
}

#[derive(Debug)]
pub enum Error {
    UnknownCodepoint,
    UnterminatedString,
    InvalidEscapeSequence,
    InvalidOperator,
    InvalidMultiLineString,
}

#[derive(Copy, Clone)]
struct NodeId {
    pub gen: NonZeroU32,
    pub offset: u32,
}

struct PrimaryExpr {
    value: Token,
}

struct UnaryExpr {
    operator: TokenKind,
    operand: NodeId,
}

struct BinaryExpr {
    operator: TokenKind,
    left: NodeId,
    right: NodeId,
}

enum Node {
    Primary(PrimaryExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    // Block()
}

impl Node {
    pub fn is_expression(&self) -> bool {
        use Node as T;

        match &self {
            | T::Primary(_)
            | T::Unary(_)
            | T::Binary(_) => true,

            _ => false,
        }
    }
}

const_assert!(size_of::<NodeId>() == 8, "Unexpected layout for NodeId");

// pub enum AstNode {
//     Expr(AstExpr),
//     Scope(AstScope),
// }

// pub struct Ast {
//     nodes: Vec<AstNode>,
// }

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

    println!("Reg: {}, Opt: {}", size_of::<NodeId>(), size_of::<Option<NodeId>>());
}

