use std::{error::Error, fs::File, io::Read, path::PathBuf};

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day6 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day6 {
    fn part1(&self) -> PartResult {
        ANSWER!(self.calc_population(80)?)
    }

    fn part2(&self) -> PartResult {
        ANSWER!(self.calc_population(256)?)
    }
}

impl Day6 {
    fn calc_population(&self, days: usize) -> Result<usize, Box<dyn Error>> {
        let mut input = String::new();
        File::open(&self.infile)?.read_to_string(&mut input)?;

        let maybe_timers: Result<Vec<usize>, _> =
            input.trim().split(',').map(str::parse).collect();

        let mut population = [0_usize; 9];
        match maybe_timers {
            Ok(timers) => {
                for t in timers {
                    population[t] += 1;
                }
            }
            _ => {
                return Err(
                    format!("invalid initial population: {}", input).into()
                )
            }
        }

        for d in 0..days {
            population[(d + 7) % 9] += population[d % 9];
        }

        Ok(population.iter().sum::<usize>())
    }
}
