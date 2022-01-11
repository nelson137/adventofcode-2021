use std::{error::Error, path::PathBuf};

use structopt::StructOpt;

use super::{todays_input, Day};

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
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let octopuses = Octopuses::from(&self.infile)?;

        let flashes: usize = octopuses.take(100).sum();
        println!("{}", flashes);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let octopuses = Octopuses::from(&self.infile)?;
        let n_octopuses = octopuses.len();

        let synchronized_step =
            octopuses.take_while(|flashes| *flashes < n_octopuses).count();
        println!("{}", synchronized_step + 1);

        Ok(())
    }
}
