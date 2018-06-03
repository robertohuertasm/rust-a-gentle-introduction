// #![allow(dead_code, unused_imports)]
extern crate rayon;

mod ownership;
mod borrowing;
mod mutable_borrowing;
mod other_things;

use ownership::test_ownership;
use borrowing::test_borrowing;
use mutable_borrowing::test_mutable_borrowing;
use other_things::*;


fn main() {
    test_ownership();
    test_borrowing();
    test_mutable_borrowing();
    // paralellism
    println!("{}", sum_of_squares(&[1,2,3,4,5]));
    // zero cost abstraction
    println!("{}", fast_blank("    "));
}