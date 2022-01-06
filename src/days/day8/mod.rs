use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::Entry;

mod util;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day8 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day8 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let entries = self.parse_entries()?;

        let mut count = 0;
        for entry in &entries {
            count += entry
                .output
                .iter()
                .filter(|o| {
                    let len = o.len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count();
        }

        println!("{}", count);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let entries = self.parse_entries()?;

        let mut answer = 0;
        for entry in &entries {
            answer += entry.solve();
        }

        println!("{}", answer);

        Ok(())
    }
}

impl Day8 {
    fn parse_entries(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);
        let entries = file
            .lines()
            .map(|l| l.map_err(Into::into).and_then(|s| s.parse()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(entries)
    }
}
