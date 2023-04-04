// #[derive(Debug, PartialEq)]
// pub struct Color {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
// }

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Number(i32),
    // BinaryOp(Box<AstNode>, Operator, Box<AstNode>),
    // UnaryOp(Operator, Box<Expression>),
}

// #[derive(PartialEq, Debug, Clone)]
// enum Operator {
//     Plus,
//     Minus,
//     Multiply,
//     Divide,
// }

// #[derive(PartialEq, Debug, Clone, Copy)]
// pub enum TokenType {
//     // Single-character tokens.
//     LeftParen,
//     RightParen,
//     LeftBrace,
//     RightBrace,
//     Comma,
//     Dot,
//     Minus,
//     Plus,
//     Semicolon,
//     Colon,
//     Slash,
//     Star,

//     // One or two character s.
//     Bang,
//     BangEqual,
//     Equal,
//     EqualEqual,
//     Greater,
//     GreaterEqual,
//     Less,
//     LessEqual,

//     // Literals.
//     Identifier,
//     String,
//     Number,

//     // Keywords.
//     And,
//     Class,
//     Else,
//     False,
//     For,
//     Fun,
//     If,
//     Nil,
//     Or,
//     Print,
//     Return,
//     Super,
//     This,
//     True,
//     Var,
//     While,

//     Error,
//     Eof,
// }
