/// Syntax data types for the Smalltalk programming language.

pub enum Syntax {
    Expr(Expr)
}

pub enum Expr {
    
}


#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Num),
    Char(char),
    Str(String),
    Symbol(String),
    Array(Vec<Literal>),
}

#[derive(Debug, PartialEq)]
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
