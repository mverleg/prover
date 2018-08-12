use logic::answer::Answer;
use util::Difficulty;
use util::Nr;
use std::fmt::Display;
use std::prelude::v1::Vec;
use num::Num;
use logic::proposition::Prop;

///
/// Create a trait for logic statements.
///
pub enum PropNum<'a, T> where T: Nr {
    Prop(&'a Prop),
    Num(&'a Num<T>),
}

pub trait Logic<T>: Difficulty + Display + Clone where T: Nr {

    fn get_props_nums<'a>() -> &'a Vec<PropNum<'a, T>>;

    fn solve(&mut self) -> Answer {
        // Get all the boolean symbols
        self.get_props_nums()
        // Assume all values for symbols

        // Check whether all true, all false, or mixed

    }
}

pub mod answer;

pub mod proposition;

pub mod not;
pub mod and;
pub mod or;

pub mod eql;
pub mod gt;
pub mod lt;
