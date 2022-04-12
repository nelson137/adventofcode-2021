use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

mod model;
use self::model::Command;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day2 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day2 {
    fn part1(&self) -> PartResult {
        let commands = self.parse_commands()?;
        let mut x = 0_isize;
        let mut y = 0_isize;

        for cmd in &commands {
            match cmd {
                Command::Forward(magnitude) => x += magnitude,
                Command::Up(magnitude) => y -= magnitude,
                Command::Down(magnitude) => y += magnitude,
            }
        }

        ANSWER!(x * y)
    }

    fn part2(&self) -> PartResult {
        let commands = self.parse_commands()?;
        let mut x = 0_isize;
        let mut y = 0_isize;
        let mut aim = 0_isize;

        for cmd in &commands {
            match cmd {
                Command::Forward(magnitude) => {
                    x += magnitude;
                    y += magnitude * aim;
                }
                Command::Up(magnitude) => aim -= magnitude,
                Command::Down(magnitude) => aim += magnitude,
            }
        }

        ANSWER!(x * y)
    }
}

impl Day2 {
    fn parse_commands(&self) -> Result<Vec<Command>, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);
        let commands = file
            .lines()
            .map(|l| match l {
                Ok(s) => s.parse(),
                _ => Err("invalid line".into()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(commands)
    }
}
