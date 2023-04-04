// use peekmore::{PeekMore, PeekMoreIterator};

// use crate::token::{Token, TokenResult, TokenType};
// use std::str::Chars;

// #[derive(Debug)]
// pub struct Scanner<'a> {
//     source: &'a String,
//     chars: PeekMoreIterator<Chars<'a>>,
//     start: usize,
//     current: usize,
//     line: i32,
// }

// impl<'a> Scanner<'a> {
//     pub fn new(source: &'a String) -> Self {
//         Scanner {
//             source,
//             chars: source.chars().peekmore(),
//             start: 0,
//             current: 0,
//             line: 1,
//         }
//     }

//     pub fn scan_token(&mut self) -> TokenResult<'a> {
//         self.skip_whitespaces();
//         self.start = self.current;
//         match self.advance() {
//             Some(c) => match c {
//                 _ if Scanner::is_alpha(c) => self.identifier(),
//                 _ if Scanner::is_digit(c) => self.number(),

//                 // Single-char tokens
//                 '(' => self.make_token(TokenType::LeftParen),
//                 ')' => self.make_token(TokenType::RightParen),
//                 '{' => self.make_token(TokenType::LeftBrace),
//                 '}' => self.make_token(TokenType::RightBrace),
//                 ';' => self.make_token(TokenType::Semicolon),
//                 ':' => self.make_token(TokenType::Colon),
//                 ',' => self.make_token(TokenType::Comma),
//                 '.' => self.make_token(TokenType::Dot),
//                 '-' => self.make_token(TokenType::Minus),
//                 '+' => self.make_token(TokenType::Plus),
//                 '/' => self.make_token(TokenType::Slash),
//                 '*' => self.make_token(TokenType::Star),

//                 // Two-char tokens
//                 '!' => self.make_token_if_matches(&'=', TokenType::BangEqual, TokenType::Bang),
//                 '=' => self.make_token_if_matches(&'=', TokenType::EqualEqual, TokenType::Equal),
//                 '<' => self.make_token_if_matches(&'=', TokenType::LessEqual, TokenType::Less),
//                 '>' => {
//                     self.make_token_if_matches(&'=', TokenType::GreaterEqual, TokenType::Greater)
//                 }

//                 // String literals
//                 '"' => self.string(),

//                 // Error
//                 _ => self.token_error(&format!("Unexpected character '{}'", c)),
//             },
//             None => self.make_eof(),
//         }
//     }

//     fn make_token_if_matches(
//         &mut self,
//         expected: &char,
//         on_match: TokenType,
//         otherwise: TokenType,
//     ) -> TokenResult<'a> {
//         if self.matches(expected) {
//             self.make_token(on_match)
//         } else {
//             self.make_token(otherwise)
//         }
//     }

//     fn make_token(&self, token_type: TokenType) -> TokenResult<'a> {
//         TokenResult {
//             line: self.line,
//             token_type,
//             data: Ok(Token {
//                 start: self.start,
//                 end: self.current,
//                 lexeme: &self.source[self.start..self.current],
//             }),
//         }
//     }

//     fn make_identifier_token(&self) -> TokenResult<'a> {
//         let lexeme = &self.source[self.start..self.current];
//         match lexeme {
//             "and" => self.make_token(TokenType::And),
//             "class" => self.make_token(TokenType::Class),
//             "else" => self.make_token(TokenType::Else),
//             "if" => self.make_token(TokenType::If),
//             "nil" => self.make_token(TokenType::Nil),
//             "or" => self.make_token(TokenType::Or),
//             "print" => self.make_token(TokenType::Print),
//             "return" => self.make_token(TokenType::Return),
//             "super" => self.make_token(TokenType::Super),
//             "var" => self.make_token(TokenType::Var),
//             "while" => self.make_token(TokenType::While),
//             "false" => self.make_token(TokenType::False),
//             "for" => self.make_token(TokenType::For),
//             "fun" => self.make_token(TokenType::Fun),
//             "this" => self.make_token(TokenType::This),
//             "true" => self.make_token(TokenType::True),
//             _ => self.make_token(TokenType::Identifier),
//         }
//     }

//     fn make_eof(&self) -> TokenResult<'a> {
//         TokenResult {
//             line: self.line,
//             token_type: TokenType::Eof,
//             data: Ok(Token {
//                 start: self.start,
//                 end: self.current,
//                 lexeme: "",
//             }),
//         }
//     }

//     fn token_error(&self, message: &str) -> TokenResult<'a> {
//         TokenResult {
//             line: self.line,
//             token_type: TokenType::Error,
//             data: Err(message.to_string()),
//         }
//     }

//     fn advance(&mut self) -> Option<char> {
//         self.current += 1;
//         self.chars.next()
//     }

//     fn peek(&mut self) -> Option<&char> {
//         self.chars.peek()
//     }

//     fn peek_next(&mut self) -> Option<&char> {
//         self.chars.peek_nth(1)
//     }

//     fn peek_matches(&mut self, expected: &char) -> bool {
//         match self.peek() {
//             Some(c) => c == expected,
//             None => false,
//         }
//     }

//     fn is_eof(&mut self) -> bool {
//         self.peek() == None
//     }

//     fn peek_next_matches(&mut self, expected: &char) -> bool {
//         match self.peek_next() {
//             Some(c) => c == expected,
//             None => false,
//         }
//     }

//     fn matches(&mut self, expected: &char) -> bool {
//         match self.peek() {
//             Some(c) => {
//                 if c == expected {
//                     self.advance();
//                     true
//                 } else {
//                     false
//                 }
//             }
//             None => false,
//         }
//     }

//     fn skip_whitespaces(&mut self) {
//         loop {
//             match self.peek() {
//                 Some(' ') | Some('\t') | Some('\r') => {
//                     self.advance();
//                 }
//                 Some('\n') => {
//                     self.line += 1;
//                     self.advance();
//                 }
//                 Some('/') => {
//                     if self.peek_next_matches(&'/') {
//                         self.advance();
//                         self.advance();
//                         loop {
//                             if self.peek_matches(&'\n') || self.is_eof() {
//                                 break;
//                             } else {
//                                 self.advance();
//                             }
//                         }
//                     } else {
//                         break;
//                     }
//                 }
//                 _ => break,
//             }
//         }
//     }

//     fn string(&mut self) -> TokenResult<'a> {
//         // I already consumed the initial " before. I'm storing as a lexeme the string
//         // with no "s
//         self.start += 1;

//         while !self.peek_matches(&'"') && !self.is_eof() {
//             if self.peek_matches(&'\n') {
//                 self.line += 1;
//             }
//             self.advance();
//         }

//         if self.is_eof() {
//             self.token_error(&format!(
//                 "Unterminated string. Token so far: {:?}",
//                 self.make_token(TokenType::String)
//             ))
//         } else {
//             let ret = self.make_token(TokenType::String);
//             self.advance();
//             ret
//         }
//     }

//     fn number(&mut self) -> TokenResult<'a> {
//         while self.peek_is_digit() {
//             self.advance();
//         }

//         if self.peek_matches(&'.') {
//             self.advance();
//             while self.peek_is_digit() {
//                 self.advance();
//             }
//         }

//         self.make_token(TokenType::Number)
//     }

//     fn peek_is_digit(&mut self) -> bool {
//         match self.peek() {
//             Some(c) => Scanner::is_digit(*c),
//             None => false,
//         }
//     }

//     fn peek_is_alpha(&mut self) -> bool {
//         match self.peek() {
//             Some(c) => Scanner::is_alpha(*c),
//             None => false,
//         }
//     }

//     fn identifier(&mut self) -> TokenResult<'a> {
//         while self.peek_is_alpha() || self.peek_is_digit() {
//             self.advance();
//         }

//         self.make_identifier_token()
//     }

//     fn is_digit(c: char) -> bool {
//         // matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
//         c.is_digit(10)
//     }

//     fn is_alpha(c: char) -> bool {
//         c == '_' || ('A'..'z').contains(&c)
//     }
// }
