use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::cmp::Ordering;

///
/// Create a trait that combines all the trait bounds.
///
pub trait Nr: Display + Clone + PartialEq + Eq + PartialOrd + Ord + From<u8> {}

impl<T> Nr for T where T: Display + Clone + PartialEq + Eq + PartialOrd + Ord + From<u8> {}


///
/// Create float types that must be defined and finite.
///
macro_rules! make_float_wrapper {
    ($name: ident, $typ: ty) => {

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        pub struct $name($typ);

        impl $name {
            pub fn new(value: $typ) -> Self {
                $name(value)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str(&format!("{}", self.0))
            }
        }

        impl Eq for $name {}

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        impl From<u8> for $name {
            fn from(nr: u8) -> Self {
                $name(nr as $typ)
            }
        }
    }
}

make_float_wrapper!(f32eq, f32);
make_float_wrapper!(f64eq, f64);

#[cfg(test)]
mod tests {
    use super::*;

    // This will give a compile error if the argument does not implement Nr.
    fn test_is_nr<T: Nr>(_: T) {}

    #[test]
    fn test_floats_nr() {
        // Note: this test fails to compile if invalid; it cannot fail once compiled.
        test_is_nr(f32eq::new(0f32));
        test_is_nr(f64eq::new(0f64));
    }
}
