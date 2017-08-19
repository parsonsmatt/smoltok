/// Syntax data types for the Smalltalk programming language.

pub enum Syntax {
    Expr(Expr)
}

pub enum Expr {
    
}

pub enum Literal {
    Number(Num),
    Char(char),
    Str(String),
    Symbol(String),
    Array(Box<Literal>),
}

pub struct Num {
    radix: Option<u8>,
    integer: String,
    mantissa: Option<String>,
    exponent: Option<String>,
}

fn mk_num(s: String) -> Num { 
    Num {
        integer: s,
        radix: None,
        mantissa: None,
        exponent: None
    }
}
