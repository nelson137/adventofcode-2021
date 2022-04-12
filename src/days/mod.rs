use std::{error::Error, fmt::Display, time::SystemTime};

use structopt::StructOpt;
use term_size::dimensions_stdout;

use crate::util::repeat_char;

pub type PartResult = Result<Box<dyn Display>, Box<dyn Error>>;

pub struct TimedSolution {
    solution: PartResult,
    time: f32,
}

impl TimedSolution {
    fn calculate(
        solver: impl FnOnce() -> PartResult,
    ) -> Result<Self, Box<dyn Error>> {
        let begin = SystemTime::now();
        let solution = solver();
        let end = SystemTime::now();
        let time = end.duration_since(begin)?.as_nanos() as f32 / 1.0e3;
        Ok(Self { solution, time })
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
    fn run_part1(&self) -> Result<TimedSolution, Box<dyn Error>> {
        TimedSolution::calculate(|| self.part1())
    }

    fn run_part2(&self) -> Result<TimedSolution, Box<dyn Error>> {
        TimedSolution::calculate(|| self.part2())
    }

    fn run(&self) -> Result<(TimedSolution, TimedSolution), Box<dyn Error>> {
        Ok((self.run_part1()?, self.run_part2()?))
    }

    fn run_and_print(&self) -> Result<(f32, f32), Box<dyn Error>> {
        let (ts1, ts2) = self.run()?;
        println!("\n=== Part 1 ===");
        ts1.print();
        println!("\n=== Part 2 ===");
        ts2.print();
        println!("");
        Ok((ts1.time, ts2.time))
    }

    fn part1(&self) -> PartResult;

    fn part2(&self) -> PartResult;
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    bench: bool,

    #[structopt(subcommand)]
    day: CliDay,
}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.bench {
            self.day.bench()
        } else {
            self.day.run()
        }
    }
}

macro_rules! decl_day {
    ($($mod:ident => $cli:ident;)+) => {
        $(
            mod $mod;
            use $mod::$cli;
        )+

        #[derive(StructOpt)]
        pub enum CliDay {
            $($cli($cli),)+
            All,
        }

        impl CliDay {
            pub fn run(&self) -> Result<(), Box<dyn Error>> {
                match self {
                    $(Self::$cli(day) => {
                        day.run_and_print()?;
                    })+
                    Self::All => {
                        let all_clis: &[Box<dyn Day>] = &[$(Box::new($cli::from_iter::<&[&str]>(&[])),)+];
                        let width = dimensions_stdout().map(|d| d.0).unwrap_or(60);
                        let mut time = 0.0;

                        println!("");

                        for (i, cli) in all_clis.iter().enumerate() {
                            print!("===[ Day {:02} ]===", i + 1);
                            println!("{}", repeat_char!('=', width - 16));
                            let (t1, t2) = cli.run_and_print()?;
                            time += t1 + t2;
                        }

                        println!("{}\n", repeat_char!('=', width));
                        println!("all answers in {} ms\n", time / 1.0e3);
                    }
                }
                Ok(())
            }

            pub fn bench(&self) -> Result<(), Box<dyn Error>> {
                const N_WARMUPS: usize = 5;
                macro_rules! avg_part_with {
                    ($runner:expr) => {{
                        let mut t = f32::MAX;
                        // Warmup
                        for _ in 0..N_WARMUPS {
                            t = t.min($runner.time);
                        }
                        let n = (1.0e6 / t) as usize;
                        if n < 3 {
                            println!("warning: part will only be run {} times", n);
                        }
                        // Sum runtimes
                        t = 0.0;
                        for _ in 0..n {
                            t += $runner.time;
                        }
                        // Average
                        t / n as f32
                    }};
                }

                match self {
                    $(Self::$cli(day) => {
                        println!("");
                        println!("Part    {:>10}", "Avg (ms)");
                        println!("--------{}", repeat_char!('-', 10));
                        let avg1 = avg_part_with!(day.run_part1()?) / 1.0e3;
                        println!("   1    {:10.4}", avg1);
                        let avg2 = avg_part_with!(day.run_part2()?) / 1.0e3;
                        println!("   2    {:10.4}", avg2);
                        println!("--------{}", repeat_char!('-', 10));
                        println!("        {:10.4}", avg1 + avg2);
                        println!("");
                    })+
                    Self::All => {
                        let all_clis: &[Box<dyn Day>] =
                            &[$(Box::new($cli::from_iter::<&[&str]>(&[]))),+];

                        println!("");

                        println!("Day    Part 1 Avg    Part 2 Avg    Total (ms)");
                        println!("---------------------------------------------");
                        let mut total = 0.0;
                        for (i, cli) in all_clis.iter().enumerate() {
                            let avg1 = avg_part_with!(cli.run_part1()?) / 1.0e3;
                            let avg2 = avg_part_with!(cli.run_part2()?) / 1.0e3;
                            let sum = avg1 + avg2;
                            total += sum;
                            println!(
                                "{:3}    {:>10.4}    {:>10.4}    {:>10.4}",
                                i + 1, avg1, avg2, sum
                            );
                        }
                        println!("---------------------------------------------");
                        println!(
                            "{:3}    {:10}    {:10}    {:10.4}",
                            " ",
                            " ",
                            " ",
                            total
                        );

                        println!("");
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
