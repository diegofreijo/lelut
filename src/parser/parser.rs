use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    error::VerboseError,
    IResult,
};

use nom::{branch::alt, sequence::preceded};

// use ast::*;
// use bytes::bytes;
// use functions::varargslist;
// use helpers::*;
// use numbers::number;
// use strings::string;

use super::ast::Expression;

// fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//     u8::from_str_radix(input, 16)
// }

// fn is_hex_digit(c: char) -> bool {
//     c.is_digit(16)
// }

// fn hex_primary(input: &str) -> IResult<&str, u8> {
//     map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
// }

// pub fn hex_color(input: &str) -> IResult<&str, Color> {
//     let (input, _) = tag("#")(input)?;
//     let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
//     Ok((input, Color { red, green, blue }))
// }

fn number(i: &str) -> IResult<&str, Expression> {
    alt((
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<i32>().map(Expression::Number)
        }),
        map(preceded(tag("-"), digit1), |digit_str: &str| {
            Expression::Number(-digit_str.parse::<i32>().unwrap())
        }),
    ))(i)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    number(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{ast::Expression, parser::number};

    #[test]
    fn numbers() {
        assert_eq!(number("0"), Ok(("", Expression::Number(0))));
        // assert_token_lexeme(String::from("4"), TokenType::Number, "4");
        // assert_token_lexeme(String::from("42"), TokenType::Number, "42");
        // assert_token_lexeme(String::from("13.99"), TokenType::Number, "13.99");
    }

    // #[test]
    // fn empty_source() {
    //     assert_eq!(String::from(""), TokenType::Eof);
    //     assert_token(String::from("    "), TokenType::Eof);
    //     assert_token(String::from("\r\t\t 	"), TokenType::Eof);
    //     assert_token(String::from("\n"), TokenType::Eof);
    // }

    // #[test]
    // fn error_source() {
    //     assert_error_token(String::from("%"));
    //     assert_error_token(String::from("@"));
    // }

    // #[test]
    // fn single_chars() {
    //     assert_token(String::from(""), TokenType::Eof);
    //     assert_token(String::from("("), TokenType::LeftParen);
    //     assert_token(String::from("}"), TokenType::RightBrace);
    //     assert_token(String::from("-"), TokenType::Minus);
    //     assert_token(String::from("+"), TokenType::Plus);
    //     assert_token(String::from("/"), TokenType::Slash);
    // }

    // #[test]
    // fn double_chars() {
    //     assert_token(String::from("=="), TokenType::EqualEqual);
    //     assert_token(String::from("!="), TokenType::BangEqual);
    //     assert_token(String::from(">"), TokenType::Greater);
    //     assert_token(String::from(">="), TokenType::GreaterEqual);
    // }

    // #[test]
    // fn full_source() {
    //     assert_tokens(String::from("+-"), &vec![TokenType::Plus, TokenType::Minus]);
    //     assert_tokens(
    //         String::from("==="),
    //         &vec![TokenType::EqualEqual, TokenType::Equal],
    //     );
    //     assert_tokens(
    //         String::from("()\n{}"),
    //         &vec![
    //             TokenType::LeftParen,
    //             TokenType::RightParen,
    //             TokenType::LeftBrace,
    //             TokenType::RightBrace,
    //         ],
    //     );
    // }

    // #[test]
    // fn coments() {
    //     assert_tokens(String::from("//pepe"), &vec![]);
    //     assert_tokens(String::from("+\n//pepe"), &vec![TokenType::Plus]);
    //     assert_tokens(String::from("/\n"), &vec![TokenType::Slash]);
    //     assert_tokens(String::from("/\n//pepe"), &vec![TokenType::Slash]);
    //     assert_tokens(
    //         String::from("/\n//pepe\n/"),
    //         &vec![TokenType::Slash, TokenType::Slash],
    //     );
    // }

    // #[test]
    // fn strings() {
    //     assert_token_lexeme(String::from("\"pepe\""), TokenType::String, "pepe");
    //     assert_token_lexeme(String::from("\"pepe\"\n"), TokenType::String, "pepe");
    //     assert_token_lexeme(String::from("\"pepe\"\n\n"), TokenType::String, "pepe");
    //     assert_token_lexeme(String::from("\"\""), TokenType::String, "");
    // }

    // #[test]
    // fn identifier() {
    //     assert_token(String::from("class"), TokenType::Class);
    //     assert_token(String::from("if"), TokenType::If);
    //     assert_token(String::from("while"), TokenType::While);
    //     assert_token(String::from("true"), TokenType::True);
    //     assert_token(String::from("false"), TokenType::False);

    //     assert_token_lexeme(String::from("pepe"), TokenType::Identifier, "pepe");
    //     assert_token_lexeme(String::from("for1"), TokenType::Identifier, "for1");
    //     assert_token_lexeme(String::from("whiles"), TokenType::Identifier, "whiles");
    // }

    // fn assert_token(source: String, expected: TokenType) {
    //     let mut scanner = scanner::Scanner::new(&source);
    //     let token = scanner.scan_token();

    //     assert_eq!(token.token_type, expected);

    //     assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    // }

    // fn assert_token_lexeme(source: String, expected_type: TokenType, expected_lexeme: &str) {
    //     let mut scanner = scanner::Scanner::new(&source);
    //     let token = scanner.scan_token();
    //     let data = token.data.unwrap();

    //     assert_eq!(token.token_type, expected_type);
    //     assert_eq!(data.lexeme, expected_lexeme);

    //     assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    // }

    // fn assert_tokens(source: String, expected_tokens: &Vec<TokenType>) {
    //     let mut scanner = scanner::Scanner::new(&source);
    //     for expected in expected_tokens {
    //         let actual = scanner.scan_token();
    //         assert_eq!(actual.token_type, *expected);
    //     }

    //     assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    // }

    // fn assert_error_token(source: String) {
    //     let mut scanner = scanner::Scanner::new(&source);
    //     let token = scanner.scan_token();

    //     assert!(token.data.is_err());
    // }
}
