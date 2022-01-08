use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::NavChunk;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day10 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day10 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let chunks = self.parse_nav_chunks()?;
        let mut stack = Vec::new();

        let mut score = 0_usize;
        for line in &chunks {
            stack.clear();
            for chunk_end in line {
                match chunk_end {
                    NavChunk::RoundOpen => stack.push(NavChunk::RoundClose),
                    NavChunk::SquareOpen => stack.push(NavChunk::SquareClose),
                    NavChunk::CurlyOpen => stack.push(NavChunk::CurlyClose),
                    NavChunk::AngledOpen => stack.push(NavChunk::AngledClose),
                    close => match stack.pop() {
                        Some(expected_close) if *close == expected_close => (),
                        _ => {
                            score += close.syntax_error_score()?;
                            break;
                        }
                    },
                }
            }
        }

        println!("{}", score);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let chunks = self.parse_nav_chunks()?;
        let mut stack = Vec::new();
        let mut scores = Vec::new();

        'line: for line in &chunks {
            stack.clear();
            let mut score = 0_usize;

            for chunk_end in line {
                match chunk_end {
                    NavChunk::RoundOpen => stack.push(NavChunk::RoundClose),
                    NavChunk::SquareOpen => stack.push(NavChunk::SquareClose),
                    NavChunk::CurlyOpen => stack.push(NavChunk::CurlyClose),
                    NavChunk::AngledOpen => stack.push(NavChunk::AngledClose),
                    close => match stack.pop() {
                        Some(expected_close) if *close == expected_close => (),
                        _ => continue 'line,
                    },
                }
            }

            while let Some(close) = stack.pop() {
                score = score * 5 + close.completion_score()?;
            }
            scores.push(score);
        }

        scores.sort();
        let answer = scores[scores.len() / 2];
        println!("{}", answer);

        Ok(())
    }
}

impl Day10 {
    fn parse_nav_chunks(&self) -> Result<Vec<Vec<NavChunk>>, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut chunks = Vec::new();

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            chunks.push(
                line.chars()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }

        Ok(chunks)
    }
}
