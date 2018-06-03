use rayon::prelude::*;

pub fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

pub extern "C" fn fast_blank(text: &str) -> bool {
    text.chars().all(|c| c.is_whitespace())
}