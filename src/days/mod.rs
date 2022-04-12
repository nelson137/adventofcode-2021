use std::{error::Error, fmt::Display};

use structopt::StructOpt;
use term_size::dimensions_stdout;

use crate::util::repeat_char;

pub type PartResult = Result<Box<dyn Display>, Box<dyn Error>>;

pub struct TimedSolution {
    solution: PartResult,
    time: f32,
}

impl TimedSolution {
    fn new(solution: PartResult, time: f32) -> Self {
        Self { solution, time }
    }

    fn print(&self) {
        match &self.solution {
            Ok(answer) => println!("{}", answer),
            Err(err) => println!("{:?}", err),
        }
        println!("[answer in {} μs]", self.time);
    }
}

pub trait Day {
    fn run(&self) -> Result<(TimedSolution, TimedSolution), Box<dyn Error>> {
        macro_rules! run_timed_part {
            ($part:expr) => {{
                let begin = std::time::SystemTime::now();
                let sol = $part;
                let end = std::time::SystemTime::now();
                let t = end.duration_since(begin)?.as_nanos() as f32 / 1.0e3;
                TimedSolution::new(sol, t)
            }};
        }

        let ts1 = run_timed_part!(self.part1());
        let ts2 = run_timed_part!(self.part2());
        Ok((ts1, ts2))
    }

    fn part1(&self) -> PartResult;

    fn part2(&self) -> PartResult;
}

macro_rules! decl_day {
    ($($mod:ident => $cli:ident;)+) => {
        $(
            mod $mod;
            use $mod::$cli;
        )+

        #[derive(StructOpt)]
        pub enum Cli {
            $($cli($cli),)+
            All,
        }

        impl Cli {
            pub fn run(&self) -> Result<(), Box<dyn Error>> {
                match self {
                    $(Self::$cli(day) => {
                        let (ts1, ts2) = day.run()?;
                        println!("\n=== Part 1 ===");
                        ts1.print();
                        println!("\n=== Part 2 ===");
                        ts2.print();
                        println!("");
                    })+
                    Self::All => {
                        let all_clis: &[Box<dyn Day>] = &[$(Box::new($cli::from_iter::<&[&str]>(&[])),)+];
                        let width = dimensions_stdout().map(|d| d.0).unwrap_or(60);
                        let mut time = 0.0;

                        println!("");

                        for (i, cli) in all_clis.iter().enumerate() {
                            print!("===[ Day {:02} ]===", i + 1);
                            println!("{}", repeat_char!('=', width - 16));
                            let (ts1, ts2) = cli.run()?;
                            println!("\n=== Part 1 ===");
                            ts1.print();
                            println!("\n=== Part 2 ===");
                            ts2.print();
                            println!("");
                            time += ts1.time + ts2.time;
                        }

                        println!("{}\n", repeat_char!('=', width));
                        println!("all answers in {} ms\n", time / 1.0e3);
                    }
                }
                Ok(())
            }
        }
    };
}

decl_day! {
    day1 => Day1;
    day2 => Day2;
    day3 => Day3;
    day4 => Day4;
    day5 => Day5;
    day6 => Day6;
    day7 => Day7;
    day8 => Day8;
    day9 => Day9;
    day10 => Day10;
    day11 => Day11;
    day12 => Day12;
    day13 => Day13;
    day14 => Day14;
}

macro_rules! todays_input {
    ($symbol:ident) => {
        lazy_static::lazy_static! {
            static ref $symbol: String = {
                let this_file = PathBuf::from(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/",
                    file!()
                ));
                match this_file.parent() {
                    Some(dir) =>
                        dir.join("input.txt").to_string_lossy().to_string(),
                    None => panic!(
                        "failed to get parent of path: {}",
                        this_file.display()
                    ),
                }
            };
        }
    };
}
pub(self) use todays_input;

macro_rules! ANSWER {
    ($value:expr) => {
        Ok(Box::new($value))
    };
}
pub(self) use ANSWER;
