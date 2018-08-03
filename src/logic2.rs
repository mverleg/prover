use num::Num;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use util::Nr;
use util::Difficulty;





pub fn eq<T: Nr>(left: Num<T>, right: Num<T>) -> BLogic<T> { Box::new(Logic::Eq(left, right)) }
pub fn gt<T: Nr>(left: Num<T>, right: Num<T>) -> BLogic<T> { Box::new(Logic::Gt(left, right)) }
pub fn lt<T: Nr>(left: Num<T>, right: Num<T>) -> BLogic<T> { Box::new(Logic::Lt(left, right)) }
//pub fn gte<T: Nr>(left: Num<T>, right: Num<T>) -> BLogic<T> { Box::new(Logic::Gte(left, right)) }
//pub fn lte<T: Nr>(left: Num<T>, right: Num<T>) -> BLogic<T> { Box::new(Logic::Lte(left, right)) }

pub fn not<T: Nr>(subject: BLogic<T>) -> BLogic<T> { Box::new(Logic::Not(subject)) }
pub fn and<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::And(left, right)) }
pub fn or<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Or(left, right)) }
//pub fn xor<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Xor(left, right)) }
//pub fn imp<T: Nr>(left: BLogic<T>, right: BLogic<T>) -> BLogic<T> { Box::new(Logic::Imp(left, right)) }

