use std::error::Error;

use structopt::StructOpt;
use term_size::dimensions_stdout;

use crate::util::repeat_char;

pub trait Day {
    fn run(&self) -> Result<f32, Box<dyn Error>> {
        macro_rules! run_timed_part {
            ($part:expr) => {{
                let begin = std::time::SystemTime::now();
                if let Err(err) = $part {
                    println!("{:?}", err);
                }
                let end = std::time::SystemTime::now();
                let t = end.duration_since(begin)?.as_micros() as f32 / 1000.0;
                println!("[answer in {} ms]", t);
                t
            }};
        }

        let mut time = 0.0;

        println!("\n=== Part 1 ===");
        time += run_timed_part!(self.part1());
        println!("\n=== Part 2 ===");
        time += run_timed_part!(self.part2());
        println!("");

        Ok(time)
    }

    fn part1(&self) -> Result<(), Box<dyn Error>>;

    fn part2(&self) -> Result<(), Box<dyn Error>>;
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
                        day.run()?;
                    })+
                    Self::All => {
                        let all_clis: &[Box<dyn Day>] = &[$(Box::new($cli::from_iter::<&[&str]>(&[])),)+];
                        let width = dimensions_stdout().map(|d| d.0).unwrap_or(60);
                        let mut time = 0.0;

                        println!("");

                        for (i, cli) in all_clis.iter().enumerate() {
                            print!("===[ Day {:02} ]===", i + 1);
                            println!("{}", repeat_char!('=', width - 16));
                            time += cli.run()?;
                        }

                        println!("{}\n", repeat_char!('=', width));
                        println!("all answers in {} ms\n", time);
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
