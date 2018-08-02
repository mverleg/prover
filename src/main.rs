use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

type BNum<T> = Box<Num<T>>;
type BLogic<T> = Box<Logic<T>>;

// TODO @mverleg: split into num and bool
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Num<T> where T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8> {
    Con(T),
    Var(String),
    Neg(BNum<T>),
    Abs(BNum<T>),
    Add(BNum<T>, BNum<T>),
    Sub(BNum<T>, BNum<T>),
    Mul(BNum<T>, BNum<T>),
    Div(BNum<T>, BNum<T>),
    Min(BNum<T>, BNum<T>),
    Max(BNum<T>, BNum<T>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Logic<T> where T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8> {
    Eq(BNum<T>, BNum<T>),
    Gt(BNum<T>, BNum<T>),
    Lt(BNum<T>, BNum<T>),
    Gte(BNum<T>, BNum<T>),
    Lte(BNum<T>, BNum<T>),
    Not(BLogic<T>),
    And(BLogic<T>, BLogic<T>),
    Or(BLogic<T>, BLogic<T>),
    Xor(BLogic<T>, BLogic<T>),
    Equiv(BLogic<T>, BLogic<T>),
    Imp(BLogic<T>, BLogic<T>),
}

impl<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>> Logic<T> {
    pub fn resolve(&self) -> Answer {
        unimplemented!();
    }
}

impl<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>> Display for Num<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&match self {
            Num::Con(value) => format!("{}", value),
            Num::Var(name) => format!("{}", name),
            Num::Neg(subject) => format!("-({})", subject),
            Num::Abs(subject) => format!("|{}|", subject),
            Num::Add(left, right) => format!("({} + {})", left, right),
            Num::Sub(left, right) => format!("({} - {})", left, right),
            Num::Mul(left, right) => format!("({} * {})", left, right),
            Num::Div(left, right) => format!("({} / {})", left, right),
            Num::Min(left, right) => format!("({} ▲ {})", left, right),
            Num::Max(left, right) => format!("({} ▼ {})", left, right),
        })
    }
}

impl<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>> Display for Logic<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&match self {
            Logic::Eq(left, right) => format!("({} = {})", left, right),
            Logic::Gt(left, right) => format!("({} > {})", left, right),
            Logic::Lt(left, right) => format!("({} < {})", left, right),
            Logic::Gte(left, right) => format!("({} ≥ {})", left, right),
            Logic::Lte(left, right) => format!("({} ≤ {})", left, right),
            Logic::Not(subject) => format!("￢({})", subject),
            Logic::And(left, right) => format!("({} ∧ {})", left, right),
            Logic::Or(left, right) => format!("({} ∨ {})", left, right),
            Logic::Xor(left, right) => format!("({} ⊕ {})", left, right),
            Logic::Equiv(left, right) => format!("({} ↔ {})", left, right),
            Logic::Imp(left, right) => format!("({} ￫ {})", left, right),
        })
    }
}

pub fn con<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(value: T) -> BNum<T> { Box::new(Num::Con(value)) }
pub fn pos<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(name: &str) -> BNum<T> { max(con(T::from(0)), var(name)) }
pub fn var<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(name: &str) -> BNum<T> { Box::new(Num::Var(name.to_owned())) }
pub fn neg<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(subject: BNum<T>) -> BNum<T> { Box::new(Num::Neg(subject)) }
pub fn abs<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(subject: BNum<T>) -> BNum<T> { Box::new(Num::Abs(subject)) }
pub fn add<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Add(left, right)) }
pub fn sub<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Sub(left, right)) }
pub fn mul<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Mul(left, right)) }
pub fn div<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Div(left, right)) }
pub fn min<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Min(left, right)) }
pub fn max<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Max(left, right)) }

pub fn eq<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Eq(left, right)) }
pub fn gt<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Gt(left, right)) }
pub fn lt<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Lt(left, right)) }
pub fn gte<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Gte(left, right)) }
pub fn lte<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Lte(left, right)) }

pub fn not<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(subject: BLogic<T>) -> BLogic<T> { Box::new(Logic::Not(subject)) }
pub fn and<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::And(left, right)) }
pub fn or<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Or(left, right)) }
pub fn xor<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Xor(left, right)) }
pub fn imp<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Imp(left, right)) }

#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    True,
    False,
    Maybe,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(match self {
            Answer::True => "true",
            Answer::False => "false",
            Answer::Maybe => "maybe",
        })
    }
}

fn main() {
    let equation = gte(mul(con(2), pos("x")), pos("x"));
    println!("{}", equation);
    println!("{}", equation.resolve());
}
