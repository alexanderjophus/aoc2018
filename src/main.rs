#![feature(test)] extern crate test;
#[macro_use] extern crate lazy_static;

mod day1;
mod day2;
mod day3;

fn main() {
    day1::solve();
    day2::solve();
    day3::solve()
}
