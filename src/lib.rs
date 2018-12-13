#![warn(
    clippy::all,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]
#![feature(vec_remove_item)]

use aoc_runner_derive::aoc_lib;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day13;

aoc_lib! { year = 2018 }
