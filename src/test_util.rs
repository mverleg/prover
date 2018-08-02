use Answer;
use Logic;
use Num;

pub fn assert_provable<T>(statement: Logic<T>) {
    let res = statement.resolve();
    if Answer::True != res {
        println!("failed to prove true statement:");
        println!("  {}", statement);
        println!("  found {}", res);
        panic!(1);
    }
}

//pub fn a() -> Num { var("α") }
//pub fn b() -> Num { var("β") }
//pub fn c() -> Num { var("γ") }
//pub fn d() -> Num { var("δ") }
//pub fn e() -> Num { var("ε") }

