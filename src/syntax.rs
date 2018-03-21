// Syntax data types for the Smoltok programming language.

/// The datatype representing valid syntax in Smoltok. Currently, we don't have
/// a type for declarations.
pub enum Syntax {
    Expr(Expr)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Id(Ident),
    Assign(Ident, Box<Expr>),
    Lit(Literal),
    Message { receiver: Box<Expr>, selector: Msg },
    Block { vars: Vec<Ident>, statements: Vec<Statement>},
    Method(Method),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MsgPat {
    Unary(Ident),
    Bin(Ident, Ident),
    Kwargs(Vec<KeyPat>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyPat {
    pub keyword: Ident,
    pub var: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    E(Expr),
    Ret(Expr)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    Unary(Ident),
    Binary(String, Box<Expr>),
    Kwargs(Vec<Keyword>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Keyword {
    pub id: Ident,
    pub val: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(Num),
    Char(char),
    Str(String),
    Symbol(String),
    Array(Vec<Literal>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Method {
    pub sig: MsgPat,
    pub temps: Option<Vec<Ident>>,
    pub stmts: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);

/// Create an Expr from a string.
///
/// # Examples
///
/// ```
/// use smoltok::syntax::*;
/// assert_eq!(
///     mk_ident_expr("hey"),
///     Expr::Id(Ident(String::from("hey")))
/// );
/// ```
pub fn mk_ident_expr(s: &str) -> Expr {
    Expr::Id(mk_ident(s))
}

pub fn mk_ident(s: &str) -> Ident {
    Ident(String::from(s))
}

#[derive(Debug, PartialEq, Clone)]
pub struct Num {
    /// Smalltalk numbers can include an optional radix to specify the base of
    /// the number. This is given as as `Nr` where `N` is the base.
    pub radix: Option<u8>,
    /// The integral part of the number is kept as a `String`. This is done to
    /// permit bases greater than 10.
    pub integer: String,
    /// For floating point numbers, the mantissa may be represented as `.N`,
    /// where `N` is some number permitted by the given base.
    pub mantissa: Option<String>,
    /// Finally, the exponent is available as `eN` where `N` is some number
    /// permitted by the given base.
    pub exponent: Option<u32>,
}

impl Num {
    /// Convenient alias for creating a base 10 integral number from a string.
    pub fn int_from_str(s: &str) -> Self {
        Num {
            integer: String::from(s),
            radix: None,
            mantissa: None,
            exponent: None
        }
    }

    pub fn to_expr(self) -> Expr {
        Expr::Lit(Literal::Number(self))
    }
}
