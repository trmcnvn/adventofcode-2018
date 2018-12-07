#![feature(drain_filter)]
#![feature(vec_remove_item)]

extern crate aoc_runner;
extern crate hashbrown;

#[macro_use]
extern crate aoc_runner_derive;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

aoc_lib! { year = 2018 }
