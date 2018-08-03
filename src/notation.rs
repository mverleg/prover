use logic::eql::Eql;
use logic::gt::Gt;
use logic::lt::Lt;
use num::Num;
use util::Nr;
use logic::not::Not;
use logic::Logic;
use logic::and::And;
use logic::or::Or;


pub fn not<T, L>(subject: L) -> Not<T, L> where L: Logic<T>, T: Nr{
    Not(subject)
}

pub fn and<T, L, R>(left: L, right: L) -> And<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    And(left, right)
}

pub fn or<T, L, R>(left: L, right: L) -> Or<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    Or(left, right)
}

pub fn xor<T, L, R>(left: L, right: L) -> Or<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    Or(And(Not(left), right), And(left, Not(right)))
}

pub fn imp<T, L, R>(left: L, right: L) -> Or<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    Or(Not(left), right)
}

pub fn rimp<T, L, R>(left: L, right: L) -> Or<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    Or(left, Not(right))
}

//

pub fn eq<T, N>(left: N, right: N) -> Eql<T, N> where N: Num<T>, T: Nr {
    Eql(left, right)
}

pub fn lt<T, N>(left: N, right: N) -> Lt<T, N> where N: Num<T>, T: Nr {
    Lt(left, right)
}

pub fn gt<T, N>(left: N, right: N) -> Gt<T, N> where N: Num<T>, T: Nr {
    Gt(left, right)
}

pub fn lte<T, N>(left: N, right: N) -> Not<T, Gt<T, N>> where N: Num<T>, T: Nr {
    Not(Gt(left, right))
}

pub fn gte<T, N>(left: N, right: N) -> Not<T, Lt<T, N>> where N: Num<T>, T: Nr {
    Not(Lt(left, right))
}

