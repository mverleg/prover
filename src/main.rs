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

pub fn con<T: Display + PartialEq + Eq>(value: T) -> BToken<T> { Box::new(Token::Con(value)) }
pub fn var<T: Display + PartialEq + Eq>(name: &str) -> BToken<T> { Box::new(Token::Var(name.to_owned())) }
pub fn neg<T: Display + PartialEq + Eq>(subject: BToken<T>) -> BToken<T> { Box::new(Token::Neg(subject)) }
pub fn abs<T: Display + PartialEq + Eq>(subject: BToken<T>) -> BToken<T> { Box::new(Token::Abs(subject)) }
pub fn add<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Add(left, right)) }
pub fn sub<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Sub(left, right)) }
pub fn mul<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Mul(left, right)) }
pub fn div<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Div(left, right)) }
pub fn gre<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Gte(left, right)) }
pub fn lte<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Lte(left, right)) }
pub fn min<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Min(left, right)) }
pub fn max<T: Display + PartialEq + Eq>(left: BToken<T>, right: BToken<T>) -> BToken<T> { Box::new(Token::Max(left, right)) }


fn main() {

}