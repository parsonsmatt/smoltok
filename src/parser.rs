/// Parser for the Smalltalk programming language.

use combine::{none_of, many, many1, try, token, optional};
use combine::Parser;
use combine::primitives::Stream;
use combine::combinator::{value, any, choice, between, sep_by};
use combine::char::{letter, digit, upper, char, string, alpha_num, spaces};

use syntax::*;

parser! {
    fn expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        literal().map(|l| Expr::Lit(l))
            .or(
                try(
                    (ident(), assignment(), expr())
                        .map(|(i, (), e)|
                            Expr::Assign(i, Box::new(e))
                             )

                )
            ).or(
                try(
                    ident().map(|i| Expr::Id(i))
                )
            )
    }
}

/// Parse an identifier.
parser! {
    fn ident[I]()(I) -> Ident
        where [I: Stream<Item = char>]
    {
        (letter(), many(alpha_num()), spaces()).map(|(c, cs, _): (char, String, ())|
            Ident(format!("{}{}", c, cs))
        )
    }
}

/// Parse assignment syntax. Smalltalk supports multiple assignment, so we
/// return a list of string identifiers
parser! {
    fn assignment[I]()(I) -> ()
        where [I: Stream<Item = char>]
    {
        ( string("<-"),
          spaces(),
        ).map(|(_, e)| e)
    }
}

/// Parse an integral number.
parser! {
    fn digits[I]()(I) -> u32
        where [I: Stream<Item = char>]
    {
        many1(digit())
            .and_then(|s: String| s.parse())
    }
}

/// Parse an uppercase character or a digit.
parser! {
    fn upper_digit[I]()(I) -> char
        where [I: Stream<Item = char>]
    {
        digit().or(upper())
    }
}

/// Parse a Smalltalk number.
parser! {
    fn number[I]()(I) -> Num
        where [I: Stream<Item = char>]
    {
        struct_parser!{
            Num {
                radix: optional(try(
                            (digits(),
                             token('r')
                            ).map(|t| t.0 as u8)
                           )),
                integer: many1(upper_digit()),
                mantissa: optional(
                    (token('.'),
                     many1(upper_digit())
                    ).map(|t| t.1)),
                exponent: optional(
                    (token('e'),
                     digits()
                    ).map(|t| t.1)
                    )
            }
        }
    }
}

/// Parse a Smalltalk character.
parser! {
    fn sm_char[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        (token('$'),
         any()
        ).map(|t| Literal::Char(t.1))
    }
}

/// Parse a Smalltalk string.
parser! {
    fn sm_string[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        (token('\''),
         many(
             none_of("'".chars())
                .or(try(string("''").map(|_| '\'' )))
             ),
         token('\'')
        ).map(|t| Literal::Str(t.1))
    }
}

/// Parses an array of literals or a symbol depending on if there is a
/// paren immediately following the hash.
parser! {
    fn sm_hash_starter[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        token('#')
            .then(|_|
                many1(alpha_num())
                    .map(|t| Literal::Symbol(t))
                    .or(
                    between(
                        token('('),
                        token(')'),
                        sep_by(literal(), spaces())
                    ).map(|t| Literal::Array(t))
                )
            )
    }
}

/// Parse any kind of Smalltalk literal. Don't worry. Just throw whatever you
/// got at it.
parser! {
    fn literal[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        spaces().then(|_| number().map(|t| Literal::Number(t))
            .or(sm_char())
            .or(sm_string())
            .or(sm_hash_starter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let ans: u32 = 10;
        let res = digits().parse("10");
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_bare_number() {
        let res = number().parse("10");
        let ans = mk_num("10");
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_exponent() {
        let res = number().parse("10e3");
        let ans = Num {
            integer: String::from("10"),
            exponent: Some(3),
            mantissa: None,
            radix: None,
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_full_number() {
        let res = number().parse("10r10.5e3");
        let ans = Num {
            integer: String::from("10"),
            exponent: Some(3),
            mantissa: Some(String::from("5")),
            radix: Some(10),
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_float() {
        let res = number().parse("123.456");
        let ans = Num {
            integer: String::from("123"),
            exponent: None,
            mantissa: Some(String::from("456")),
            radix: None,
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_radix() {
        let res = number().parse("16rAC.DCe10");
        let ans = Num {
            integer: String::from("AC"),
            exponent: Some(10),
            mantissa: Some(String::from("DC")),
            radix: Some(16),
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_char() {
        let res = sm_char().parse("$a");
        let ans = Literal::Char('a');
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_string() {
        let res = sm_string().parse("'hello world'");
        let ans = Literal::Str(String::from("hello world"));
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_string_quotes() {
        let res = sm_string().parse("'hello ''world'''");
        let ans = Literal::Str(String::from("hello 'world'"));
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_symbol() {
        let res = sm_hash_starter().parse("#foobar123");
        let ans = Literal::Symbol(String::from("foobar123"));
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_literal() {
        let res = literal().parse("#('hello' 123 #world)");
        let ans = Literal::Array(vec![
            Literal::Str(String::from("hello")),
            Literal::Number(mk_num("123")),
            Literal::Symbol(String::from("world")),
        ]);
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_ident() {
        let res = ident().parse("index");
        let ans = mk_ident("index");
        assert_eq!(res, Ok((ans, "")))
    }

    #[test]
    fn test_single_assignment() {
        let res = expr().parse("foo <- bar");
        let ans = Expr::Assign(mk_ident("foo"), Box::new(mk_ident_expr("bar")));
        assert_eq!(res, Ok((ans, "")))
    }

    #[test]
    fn test_expr_assigment() {
        let res = expr().parse("foo <- 'hello world'");
        let ans = Expr::Assign(
            mk_ident("foo"),
            Box::new(Expr::Lit(Literal::Str(String::from("hello world")))),
        );
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_assign_number() {
        let res = expr().parse("foo <- 3r2e3");
        let ans = Expr::Assign(
            mk_ident("foo"),
            Box::new(Expr::Lit(Literal::Number(Num {
                radix: Some(3),
                integer: String::from("2"),
                mantissa: None,
                exponent: Some(3),
            }))),
        );
        assert_eq!(res, Ok((ans, "")));
    }
}
