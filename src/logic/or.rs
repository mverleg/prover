

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Or<L>(Logic<T>, Logic<T>) where L: Logic<T>, T: Nr;

impl<L> Difficulty for Or<L> where L: Logic<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<L> Logic for Or<L> where L: Logic<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T> Display for Or<T> where T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(format!(&"({} ∨ {})", self.0, self.1))
    }
}
