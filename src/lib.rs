use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use num::BNum;

type BLogic<T> = Box<Logic<T>>;

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

pub mod logic;
pub mod num;
pub mod test_util;

#[cfg(test)]
mod tests {
    use super::*;
    use super::num::*;
    use super::logic::*;
    use super::test_util::assert_provable;

    #[test]
    fn test_triangle_inequality() {
        assert_provable::<i32>(lte(
            abs(add(var("x"), var("y"))),
            add(abs(var("x")), abs(var("y"))),
        ));
    }

    #[test]
    fn test_twice_min() {
        assert_provable(gte(
            mul(con(2), pos("x")),
            pos("x")
        ));
    }
}
