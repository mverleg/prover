use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

type BToken<T> = Box<Token<T>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<T> where T: Display + PartialEq + Eq {
    Con(T),
    Var(String),
    Neg(BToken<T>),
    Abs(BToken<T>),
    Add(BToken<T>, BToken<T>),
    Sub(BToken<T>, BToken<T>),
    Mul(BToken<T>, BToken<T>),
    Div(BToken<T>, BToken<T>),
    Gte(BToken<T>, BToken<T>),
    Lte(BToken<T>, BToken<T>),
    Min(BToken<T>, BToken<T>),
    Max(BToken<T>, BToken<T>),
}

impl <T: Display + PartialEq + Eq> Display for Token<T> {
    fn fmt(&self, f: & mut Formatter) -> Result<(), Error> {
        f.write_str(&match self {
            Token::Con(value) => format!("{}", value),
            Token::Var(name) => format!("{}", name),
            Token::Neg(subject) => format!("-({})", subject),
            Token::Abs(subject) => format!("|{}|", subject),
            Token::Add(left, right) => format!("({} + {})", left, right),
            Token::Sub(left, right) => format!("({} - {})", left, right),
            Token::Mul(left, right) => format!("({} * {})", left, right),
            Token::Div(left, right) => format!("({} / {})", left, right),
            Token::Gte(left, right) => format!("({} ≥ {})", left, right),
            Token::Lte(left, right) => format!("({} ≤ {})", left, right),
            Token::Min(left, right) => format!("({} ∧ {})", left, right),
            Token::Max(left, right) => format!("({} ∨ {})", left, right),
        })
    }
}

fn main() {

}