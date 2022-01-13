use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day14 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day14 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        self.parse()?;
        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Day14 {
    fn parse(&self) -> Result<(), Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            println!("{}", line);
        }

        Ok(())
    }
}
