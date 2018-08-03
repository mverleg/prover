//use std::fmt::Display;
//use std::fmt::Error;
//use std::fmt::Formatter;
//use util::Nr;
//
//// TODO @mverleg: ALL OF THIS IS STILL TO BE SPLIT INTO STRUCTS AND FILES
//
//#[derive(Debug, Clone, PartialEq, Eq)]
//pub enum Num<T> where T: Nr {
//    Con(T),
//    Var(String),
//    Neg(BNum<T>),
//    Abs(BNum<T>),
//    Add(BNum<T>, BNum<T>),
//    Sub(BNum<T>, BNum<T>),
//    Mul(BNum<T>, BNum<T>),
//    Div(BNum<T>, BNum<T>),
//    Min(BNum<T>, BNum<T>),
//    Max(BNum<T>, BNum<T>),
//}
//
//impl<T: Nr> Display for Num<T> {
//    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//        f.write_str(&match self {
//            Num::Con(value) => format!("{}", value),
//            Num::Var(name) => format!("{}", name),
//            Num::Neg(subject) => format!("-({})", subject),
//            Num::Abs(subject) => format!("|{}|", subject),
//            Num::Add(left, right) => format!("({} + {})", left, right),
//            Num::Sub(left, right) => format!("({} - {})", left, right),
//            Num::Mul(left, right) => format!("({} * {})", left, right),
//            Num::Div(left, right) => format!("({} / {})", left, right),
//            Num::Min(left, right) => format!("({} ▲ {})", left, right),
//            Num::Max(left, right) => format!("({} ▼ {})", left, right),
//        })
//    }
//}
//
//pub fn con<T: Nr>(value: T) -> BNum<T> { Box::new(Num::Con(value)) }
//pub fn var<T: Nr>(name: &str) -> BNum<T> { Box::new(Num::Var(name.to_owned())) }
//pub fn neg<T: Nr>(subject: BNum<T>) -> BNum<T> { Box::new(Num::Neg(subject)) }
//pub fn abs<T: Nr>(subject: BNum<T>) -> BNum<T> { Box::new(Num::Abs(subject)) }
//pub fn add<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Add(left, right)) }
//pub fn sub<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Sub(left, right)) }
//pub fn mul<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Mul(left, right)) }
//pub fn div<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Div(left, right)) }
//pub fn min<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Min(left, right)) }
//pub fn max<T: Nr>(left: BNum<T>, right: BNum<T>) -> BNum<T> { Box::new(Num::Max(left, right)) }
//
//pub fn pos<T: Nr>(name: &str) -> BNum<T> { max(con(T::from(0)), var(name)) }
//pub fn sq<T: Nr>(subject: BNum<T>) -> BNum<T> { mul(subject.clone(), subject) }
//
//pub fn a<T: Nr>() -> BNum<T> { var("α") }
//pub fn b<T: Nr>() -> BNum<T> { var("β") }
//pub fn c<T: Nr>() -> BNum<T> { var("γ") }
//pub fn d<T: Nr>() -> BNum<T> { var("δ") }
//pub fn e<T: Nr>() -> BNum<T> { var("ε") }
