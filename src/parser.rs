/// Parser for the Smalltalk programming language.

extern crate combine;

use syntax::*;

/// Parse a number in the weird Smalltalk format.
pub fn number() -> Num {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_number() {
        let res = Num { 
            radix: None,  
            integer : "10",
            mantissa : None,
            exponent : None,
        };
        assert_eq!(4, 2); 
    }

}
