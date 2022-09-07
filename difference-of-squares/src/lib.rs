use std::ops::Add;

pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).fold(0, Add::add).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|i| i.pow(2)).fold(0, Add::add)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
