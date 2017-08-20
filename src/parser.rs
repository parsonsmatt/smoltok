/// Parser for the Smalltalk programming language.

use combine::{many1, try, token, optional};
use combine::Parser;
use combine::primitives::{Stream};
use combine::combinator::With;
use combine::combinator::{and_then};
use combine::char::{digit, letter, spaces, char, alpha_num, upper};

use syntax::*;

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
        let ans = mk_num(String::from("10"));
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
}
