/// Parser for the Smalltalk programming language.

use combine::{none_of, many, many1, try, token, optional};
use combine::Parser;
use combine::primitives::Stream;
use combine::combinator::{one_of, any, between, sep_by, value};
use combine::char::{letter, digit, upper, string, alpha_num, spaces};

use syntax::*;

parser! {
    fn expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        try(
            (ident(), assignment(), expr())
                .map(|t| Expr::Assign(t.0, Box::new(t.2)))
           ).or(try(cascaded_message_expr()))
            .or(try(message_expr()))
            .or(primary())
    }
}

parser! {
    fn unary_object[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        primary().or(unary_expr())
    }
}

parser! {
    fn unary_expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        (unary_object(), unary_selector())
            .map(|(o, s)|
                 Expr::Message {
                     receiver: Box::new(o),
                     selector: s
                 }
            )
    }
}

parser! {
    fn unary_selector[I]()(I) -> Msg
        where [I: Stream<Item = char>]
    {
        ident().map(Msg::Unary)
    }
}

parser! {
    fn binary_object[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        spaces().then(|_| unary_object().or(binary_expr()))
    }
}

parser! {
    fn binary_expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        (binary_object(), binary_selector(), unary_object())
            .map(|(bin_o, bin_sel, obj)|
                Expr::Message {
                    receiver: Box::new(bin_o),
                    selector: Msg::Binary(bin_sel, Box::new(obj))
                }
            )
    }
}

parser! {
    fn keyword_expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        (binary_object(),
         many1(
             (keyword_lit(), binary_object(), spaces())
                .map(|(s, o, _)| Keyword {
                    id: Ident(s),
                    val: o
                })
         )
        ).map(|(bin_obj, exprs): (_, Vec<Keyword>)|
                 Expr::Message {
                     receiver: Box::new(bin_obj),
                     selector: Msg::Kwargs(exprs)
                 }
            )
    }
}

parser! {
    fn message_expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        try(keyword_expr())
            .or(try(binary_expr()))
            .or(unary_expr())
    }
}

parser! {
    fn cascaded_message_expr[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        let next = (
            optional(token(';')),
            unary_selector()
                .or(
                    (binary_selector(), unary_object())
                        .map(|(bin_sel, expr)|
                             Msg::Binary(bin_sel, Box::new(expr))
                        )
                ).or(
                    many1(
                        (keyword_lit(), binary_object())
                            .map(|(id, val)| Keyword { id: Ident(id), val })
                    ).map(Msg::Kwargs)
                )
        ).map(|t| t.1);
        (message_expr(), many1(next))
            .map(|(a, b): (_, Vec<Msg>)| {
                b.iter().fold(a, |acc, msg| Expr::Message {
                    receiver: Box::new(acc),
                    selector: msg.clone()
                })
            })
    }
}

parser! {
    fn keyword_lit[I]()(I) -> String
        where [I: Stream<Item = char>]
    {
        (ident(), token(':'), spaces()).map(|(Ident(i), _, _)| format!("{}:", i))
    }
}

parser! {
    fn primary[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        ident().map(Expr::Id)
            .or(literal().map(Expr::Lit))
            .or(block())
            .or(
                    between(
                        (token('('), spaces()),
                        token(')'),
                        expr()
                    )
            )
    }
}

parser! {
    fn block[I]()(I) -> Expr
        where [I: Stream<Item = char>]
    {
        between(
            (token('['), spaces()),
            token(']'),
            (block_vars(), token('|'), spaces(), statements())
                .map(|(vars, _, _, statements)| Expr::Block { vars, statements })
            )
    }
}

parser! {
    fn statements[I]()(I) -> Vec<Statement>
        where [I: Stream<Item = char>]
    {
        (token('^'), spaces(), expr()).map(|(_, _, e)| vec![Statement::Ret(e)])
            .or(
                ((expr(), token('.'), spaces(), statements())
                    .map(|(e, _, _, s)| {
                        let mut m = Vec::new();
                        m.push(Statement::E(e));
                        m.extend(s);
                        m
                    })
            )).or(value(vec![]))
    }
}


parser! {
    fn block_vars[I]()(I) -> Vec<Ident>
        where [I: Stream<Item = char>]
    {
        many1((token(':'), ident()).map(|t| t.1))
    }
}


/// Parse an identifier.
parser! {
    fn ident[I]()(I) -> Ident
        where [I: Stream<Item = char>]
    {
        (letter(), many(alpha_num()), spaces()).map(|(c, cs, _): (char, String, _)|
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

parser! {
    fn array[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        between(
            token('('),
            token(')'),
            sep_by(
                number().map(Literal::Number)
                    .or(symbol())
                    .or(sm_string())
                    .or(sm_char())
                    .or(array()),
                spaces()
            )
        ).map(Literal::Array)
    }
}

parser! {
    fn symbol[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        ident().map(|Ident(i)| Literal::Symbol(i))
            .or(binary_selector().map(Literal::Symbol))
            .or(
                many1(keyword_lit())
                    .map(|kws: Vec<_>| Literal::Symbol(kws.join("")))
            )

    }
}

parser! {
    fn binary_selector[I]()(I) -> String
        where [I:Stream<Item = char>]
    {
        spaces().then(|_| (special_char(), optional(special_char()), spaces())
            .or(token('-').map(|t| (t, None, ())))
            .map(|(c, mc, _)| match mc {
                Some(x) => format!("{}{}", c, x),
                None => format!("{}", c)
            }))

    }
}

parser! {
    fn special_char[I]()(I) -> char
        where [I:Stream<Item = char>]
    {
        one_of("+/\\*~<>=@%|&?!".chars())
    }
}

/// Parse any kind of Smalltalk literal. Don't worry. Just throw whatever you
/// got at it.
parser! {
    fn literal[I]()(I) -> Literal
        where [I:Stream<Item = char>]
    {
        spaces().then(|_| number().map(Literal::Number)
            .or(sm_char())
            .or(sm_string())
            .or((token('#'), array().or(symbol())).map(|t| t.1))
        )
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
        let res = literal().parse("#foobar123");
        let ans = Literal::Symbol(String::from("foobar123"));
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_literal() {
        let res = literal().parse("#('hello' 123 world)");
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

    #[test]
    fn test_multiple_assignment() {
        let res = expr().parse("foo <- bar <- 'hello world'");
        let ans = Expr::Assign(
            mk_ident("foo"),
            Box::new(Expr::Assign(
                mk_ident("bar"),
                Box::new(
                    Expr::Lit(Literal::Str(String::from("hello world"))),
                ),
            )),
        );
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_unary_message_expr() {
        let res = expr().parse("theta sin");
        let ans = Expr::Message {
            receiver: Box::new(mk_ident_expr("theta")),
            selector: Msg::Unary(mk_ident("sin")),
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_binary_expr_num() {
        let res = binary_expr().parse("3 + 2");
        let ans = Expr::Message {
            receiver: Box::new(Expr::Lit(Literal::Number(mk_num("3")))),
            selector: Msg::Binary(String::from("+"), Box::new(Expr::Lit(Literal::Number(mk_num("2")))))
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_binary_expr() {
        let res = expr().parse("foo + 2");
        let ans = Expr::Message {
            receiver: Box::new(mk_ident_expr("foo")),
            selector: Msg::Binary(String::from("+"), Box::new(Expr::Lit(Literal::Number(mk_num("2")))))
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_keyword_message() {
        let res = expr().parse("a b: 2");
        let ans = Expr::Message {
            receiver: Box::new(mk_ident_expr("a")),
            selector: Msg::Kwargs(vec![
                Keyword {
                    id: mk_ident("b:"),
                    val: Expr::Lit(Literal::Number(mk_num("2")))
                },
            ])
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_keyword_messages() {
        let res = expr().parse("a b: 2 c: 3");
        let ans = Expr::Message {
            receiver: Box::new(mk_ident_expr("a")),
            selector: Msg::Kwargs(vec![
                Keyword {
                    id: mk_ident("b:"),
                    val: Expr::Lit(Literal::Number(mk_num("2")))
                },
                Keyword {
                    id: mk_ident("c:"),
                    val: Expr::Lit(Literal::Number(mk_num("3")))
                },
            ])
        };
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_many_unary_messages() {
        let res = expr().parse("theta sin round");
        let ans = Expr::Message {
            receiver: Box::new(Expr::Message {
                receiver: Box::new(mk_ident_expr("theta")),
                selector: Msg::Unary(mk_ident("sin")),
            }),
            selector: Msg::Unary(mk_ident("round")),
        };
        assert_eq!(res, Ok((ans, "")));
    }

//    #[test]
//    fn test_empty_statements() {
//        let res = statements().parse("");
//        let ans = vec![];
//        assert_eq!(res, Ok((ans, "")));
//    }

    #[test]
    fn test_return_statement() {
        let res = statements().parse("^ 'foo'");
        let ans = vec![Statement::Ret(Expr::Lit(Literal::Str(String::from("foo"))))];
        assert_eq!(res, Ok((ans, "")));
    }

    #[test]
    fn test_many_statements() {
        let res = statements().parse("foo <- bar. ^ foo");
        let ans = vec![
            Statement::E(Expr::Assign(
                mk_ident("foo"),
                Box::new(mk_ident_expr("bar")),
            )),
            Statement::Ret(mk_ident_expr("foo")),
        ];
        assert_eq!(res, Ok((ans, "")));
    }
}
