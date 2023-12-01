mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

use crate::{run_problem, AocCache, Result};
use anyhow::bail;
use std::time::Duration;

fn do_day(c: &AocCache, day: u32) -> Result<Duration> {
    match day {
        1 => run_problem::<d01::Day>(c),
        2 => run_problem::<d02::Day>(c),
        3 => run_problem::<d03::Day>(c),
        4 => run_problem::<d04::Day>(c),
        5 => run_problem::<d05::Day>(c),
        6 => run_problem::<d06::Day>(c),
        7 => run_problem::<d07::Day>(c),
        8 => run_problem::<d08::Day>(c),
        9 => run_problem::<d09::Day>(c),
        10 => run_problem::<d10::Day>(c),
        11 => run_problem::<d11::Day>(c),
        12 => run_problem::<d12::Day>(c),
        13 => run_problem::<d13::Day>(c),
        14 => run_problem::<d14::Day>(c),
        15 => run_problem::<d15::Day>(c),
        16 => run_problem::<d16::Day>(c),
        17 => run_problem::<d17::Day>(c),
        18 => run_problem::<d18::Day>(c),
        19 => run_problem::<d19::Day>(c),
        20 => run_problem::<d20::Day>(c),
        21 => run_problem::<d21::Day>(c),
        22 => run_problem::<d22::Day>(c),
        23 => run_problem::<d23::Day>(c),
        24 => run_problem::<d24::Day>(c),
        25 => run_problem::<d25::Day>(c),
        _ => bail!("Invalid day."),
    }
}

pub fn do_problems(c: &AocCache, problems: Vec<u32>) -> Result<Duration> {
    problems
        .iter()
        .try_fold(Duration::ZERO, |dur, day| Ok(dur + do_day(c, *day)?))
}
