/// Syntax data types for the Smalltalk programming language.

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

pub fn mk_num(s: &str) -> Num {
    Num {
        integer: String::from(s),
        radix: None,
        mantissa: None,
        exponent: None
    }
}
