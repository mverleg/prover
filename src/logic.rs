use num::BNum;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use util::Nr;

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

///
/// Create a trait for logic statements.
///
pub trait Logic {
    fn solve(&mut self) -> Answer;

    fn difficulty(&mut self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not<T>(Logic<T>) where T: Nr;

impl<T> Logic for Not<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for Not<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"￢({})", self.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eql<T, L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Logic for Eql<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for Eql<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} = {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gt<T, L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Logic for Gt<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for Gt<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} > {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lt<T, L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Logic for Lt<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for Lt<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} < {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And<T, L>(Logic<T>, Logic<T>) where L: Logic<T>, T: Nr;

impl<T> Logic for And<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for And<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} ∧ {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Or<T, L>(Logic<T>, Logic<T>) where L: Logic<T>, T: Nr;

impl<T> Logic for Or<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }

    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Display for Or<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} ∨ {})", self.0, self.1))
    }
}

pub fn eq<T: Nr>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Eq(left, right)) }
pub fn gt<T: Nr>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Gt(left, right)) }
pub fn lt<T: Nr>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Lt(left, right)) }
//pub fn gte<T: Nr>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Gte(left, right)) }
//pub fn lte<T: Nr>(left: BNum<T>, right: BNum<T>) -> BLogic<T> { Box::new(Logic::Lte(left, right)) }

pub fn not<T: Nr>(subject: BLogic<T>) -> BLogic<T> { Box::new(Logic::Not(subject)) }
pub fn and<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::And(left, right)) }
pub fn or<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Or(left, right)) }
//pub fn xor<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Xor(left, right)) }
//pub fn imp<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Imp(left, right)) }

