use std::{error::Error, fs::File, io::Read, path::PathBuf};

use structopt::StructOpt;

use super::{todays_input, Day};

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day7 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day7 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let mut crabs = self.parse_crap_positions()?;
        crabs.sort();

        let median = crabs[crabs.len() / 2];

        let answer: usize = crabs
            .iter()
            .map(|c| (*c as isize - median as isize).abs() as usize)
            .sum();

        println!("{}", answer);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let crabs = self.parse_crap_positions()?;

        macro_rules! cost {
            ($x:expr) => {{
                crabs
                    .iter()
                    .map(|c| {
                        let dx = (*c as isize - $x as isize).abs() as usize;
                        dx * (dx + 1) / 2 // (0..dx).sum()
                    })
                    .sum::<usize>()
            }};
        }

        let mean = (crabs.iter().sum::<usize>() as f32 / crabs.len() as f32)
            .round() as usize;
        let mut min_cost = cost!(mean);

        let approx_range = crabs.len() / 8;
        let approx_begin = mean.saturating_sub(approx_range / 2);
        let approx_end = approx_begin + approx_range;

        // Check if there is a lower cost on the left
        for m in approx_begin..mean {
            let cost = cost!(m);
            if cost < min_cost {
                min_cost = cost;
            }
        }

        // Check if there is a lower cost on the right
        for m in mean..approx_end {
            let cost = cost!(m);
            if cost < min_cost {
                min_cost = cost;
            }
        }

        println!("{}", min_cost);

        Ok(())
    }
}

impl Day7 {
    fn parse_crap_positions(&self) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut input = String::new();
        File::open(&self.infile)?.read_to_string(&mut input)?;
        input
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| format!("invalid crab positions: {}", input).into())
    }
}
