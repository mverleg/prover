use num::BNum;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use util::Nr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eql<L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Difficulty for _ where T: Nr {
        fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Logic for Eql<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T> Display for Eql<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} = {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gt<T, L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Difficulty for _ where T: Nr {
        fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Logic for Gt<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T> Display for Gt<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} > {})", self.0, self.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lt<T, L>(BNum<T>, BNum<T>) where L: Logic<T>, T: Nr;

impl<T> Difficulty for _ where T: Nr {
        fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T> Logic for Lt<T> where T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T> Display for Lt<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} < {})", self.0, self.1))
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

