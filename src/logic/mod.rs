use logic::answer::Answer;
use util::Difficulty;
use util::Nr;
use std::fmt::Display;

///
/// Create a trait for logic statements.
///
pub trait Logic<T>: Difficulty + Display + Clone where T: Nr {
    fn solve(&mut self) -> Answer;
}

pub mod answer;

pub mod proposition;

pub mod not;
pub mod and;
pub mod or;

pub mod eql;
pub mod gt;
pub mod lt;
