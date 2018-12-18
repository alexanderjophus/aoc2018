#![feature(test)] extern crate test;
#[macro_use] extern crate lazy_static;

mod day01;
mod day02;
mod day03;

fn main() {
    day01::solve();
    day02::solve();
    day03::solve()
}
