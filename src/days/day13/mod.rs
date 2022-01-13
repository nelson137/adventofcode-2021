use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::{Instruction, Paper};

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day13 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day13 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let (mut paper, instructions) = self.parse_instructions()?;

        paper.fold(&instructions[0]);

        let answer = paper.count_dots();
        println!("{}", answer);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let (mut paper, instructions) = self.parse_instructions()?;

        for ins in &instructions {
            paper.fold(ins);
        }

        print!("{}", paper);

        Ok(())
    }
}

impl Day13 {
    fn parse_instructions(
        &self,
    ) -> Result<(Paper, Vec<Instruction>), Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);
        let mut line_results = file.lines();

        let mut coords = Vec::<(usize, usize)>::new();

        while let Some(line_res) = line_results.next() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;

            if line.is_empty() {
                break;
            }

            let mut xy = line.split(',');

            match (xy.next(), xy.next(), xy.next()) {
                (Some(x), Some(y), None) =>
                    coords.push((x.trim().parse()?, y.trim().parse()?)),
                _ => return Err(format!("invalid line: {}", line).into()),
            };
        }

        let paper = Paper::from_coords(coords)?;

        let mut instructions = Vec::<Instruction>::new();

        while let Some(line_res) = line_results.next() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            instructions.push(line.parse()?);
        }

        Ok((paper, instructions))
    }
}
