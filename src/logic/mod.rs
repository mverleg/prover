
///
/// Create a trait for logic statements.
///
pub trait Logic: Difficulty {
    fn solve(&mut self) -> Answer;
}

pub mod answer;

pub mod not;
pub mod and;
pub mod or;

pub mod eql;
pub mod gt;
pub mod lt;
