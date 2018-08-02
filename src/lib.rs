
pub mod logic;
pub mod num;
pub mod test_util;

#[cfg(test)]
mod tests {
    use super::num::*;
    use super::logic::*;
    use super::test_util::assert_provable;

    #[test]
    fn test_triangle_inequality() {
        assert_provable::<i32>(lte(
            abs(add(var("x"), var("y"))),
            add(abs(var("x")), abs(var("y"))),
        ));
    }

    #[test]
    fn test_twice_min() {
        assert_provable(gte(
            mul(con(2), pos("x")),
            pos("x")
        ));
    }
}
