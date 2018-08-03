
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not<L>(L) where L: Logic<T>, T: Nr;

impl<L> Difficulty for Not<L> where L: Logic<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty()
    }
}

impl<L> Logic for Not<L> where L: Logic<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<L, T> Display for Not<L> where L: Logic<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"￢({})", self.0))
    }
}
