use crate::util::constant_setting::ConstantValue;
use std::ops::AddAssign;

pub fn get_value_and_increase<T: ConstantValue + Clone + AddAssign>(x: &mut T) -> T {
    let a = x.clone();
    *x += T::ONE;
    a
}