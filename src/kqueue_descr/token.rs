use libc::c_int;

#[derive(PartialEq, Eq)]
pub struct Token(pub c_int);

impl From<c_int> for Token {
    fn from(val: c_int) -> Token {
        Token(val)
    }
}

impl From<Token> for c_int {
    fn from(val: Token) -> c_int {
        val.0
    }
}
