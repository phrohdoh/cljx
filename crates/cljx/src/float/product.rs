use ::std::iter;
use crate::prelude::*;

impl iter::Product<Float> for Float {
    fn product<I: Iterator<Item = Float>>(iter: I) -> Self {
        iter.fold(Float::from(1.0), |acc, x| acc * x)
    }
}

impl<'a> iter::Product<&'a Float> for Float {
    fn product<I: Iterator<Item = &'a Float>>(iter: I) -> Self {
        iter.fold(Float::from(1.0), |acc, x| acc * x)
    }
}
