
pub mod logic;
pub mod num;
pub mod test_util;
pub mod util;

#[cfg(test)]
#[allow(unreachable_code)]
mod tests {
    use super::num::*;
    use super::logic::*;
    use super::test_util::assert_provable;

    #[test]
    fn test_numbers() {
        assert_provable(xor(
            // (2 * 3 + 4 > 9) xor (3 * 3 = 2 * 4)
            gt(add(mul(con(2), con(3)), con(4)), con(9)),
            eq(sq(con(3)), mul(con(2), con(4))),
        ));
    }

    #[test]
    fn test_triangle_inequality() {
        return;  // TODO: enable when implemented
        assert_provable::<i32>(lte(
            abs(add(var("x"), var("y"))),
            add(abs(var("x")), abs(var("y"))),
        ));
    }

    #[test]
    fn test_twice_min() {
        return;  // TODO: enable when implemented
        assert_provable(gte(
            mul(con(2), pos("x")),
            pos("x")
        ));
    }
}
