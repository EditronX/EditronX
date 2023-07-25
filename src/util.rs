use std::{cmp::PartialOrd, ops::Sub};

pub fn subtract<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if x > y {
        x - y
    } else {
        y - x
    }
}
