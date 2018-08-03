use logic::and::And;
use logic::eql::Eql;
use logic::gt::Gt;
use logic::Logic;
use logic::lt::Lt;
use logic::not::Not;
use logic::or::Or;
use logic::proposition::Prop;
use num::Num;
use util::Nr;
use std::marker::PhantomData;


pub fn a() -> Prop { Prop("a".to_owned()) }
pub fn b() -> Prop { Prop("b".to_owned()) }
pub fn c() -> Prop { Prop("c".to_owned()) }
pub fn d() -> Prop { Prop("d".to_owned()) }
pub fn e() -> Prop { Prop("e".to_owned()) }

pub fn not<T, L>(subject: L) -> Not<T, L> where L: Logic<T>, T: Nr{
    Not(subject, PhantomData)
}

pub fn and<T, L, R>(left: L, right: R) -> And<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    And(left, right, PhantomData)
}

pub fn or<T, L, R>(left: L, right: R) -> Or<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    Or(left, right, PhantomData)
}

pub fn xor<T, L, R>(left: L, right: R) -> Or<T, And<T, Not<T, L>, R>, And<T, L, Not<T, R>>> where L: Logic<T>, R: Logic<T>, T: Nr {
    or(and(not(left.clone()), right.clone()), and(left, not(right)))
}

pub fn imp<T, L, R>(left: L, right: R) -> Or<T, Not<T, L>, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    or(not(left), right)
}

pub fn rimp<T, L, R>(left: L, right: R) -> Or<T, L, Not<T, R>> where L: Logic<T>, R: Logic<T>, T: Nr {
    or(left, not(right))
}

//
//
//pub fn eq<T, L, R>(left: N, right: N) -> Eql<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
//    Eql(left, right)
//}
//
//pub fn lt<T, L, R>(left: N, right: N) -> Lt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
//    Lt(left, right)
//}
//
//pub fn gt<T, L, R>(left: N, right: N) -> Gt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
//    Gt(left, right)
//}
//
//pub fn lte<T, L, R>(left: N, right: N) -> Not<T, Gt<T, L, R>> where L: Num<T>, R: Num<T>, T: Nr {
//    Not(Gt(left, right))
//}
//
//pub fn gte<T, L, R>(left: N, right: N) -> Not<T, Lt<T, L, R>> where L: Num<T>, R: Num<T>, T: Nr {
//    Not(Lt(left, right))
//}
//
