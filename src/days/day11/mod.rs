use std::path::PathBuf;

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

mod model;
use self::model::Octopuses;

mod util;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day11 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day11 {
    fn part1(&self) -> PartResult {
        let octopuses = Octopuses::from(&self.infile)?;
        ANSWER!(octopuses.take(100).sum::<usize>())
    }

    fn part2(&self) -> PartResult {
        let octopuses = Octopuses::from(&self.infile)?;
        let n_octopuses = octopuses.len();
        let synchronized_step =
            octopuses.take_while(|flashes| *flashes < n_octopuses).count();
        ANSWER!(synchronized_step + 1)
    }
}
