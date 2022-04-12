use std::{
    cmp::max,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

mod model;
use self::model::Vent;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day5 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day5 {
    fn part1(&self) -> PartResult {
        let (width, height, vents) = self.parse_vents()?;

        let mut diagram: Vec<Vec<usize>> = vec![vec![0_usize; width]; height];

        for ls in &vents {
            match *ls {
                Vent::Horizontal { x1, x2, y } => {
                    for point in &mut diagram[y][x1..=x2] {
                        *point += 1;
                    }
                }
                Vent::Vertical { x, y1, y2 } => {
                    for row in &mut diagram[y1..=y2] {
                        row[x] += 1;
                    }
                }
                Vent::Diagonal { .. } => (),
            }
        }

        ANSWER!(count_overlapping_vents(&diagram))
    }

    fn part2(&self) -> PartResult {
        let (width, height, vents) = self.parse_vents()?;

        let mut diagram: Vec<Vec<usize>> = vec![vec![0_usize; width]; height];

        for ls in &vents {
            match *ls {
                Vent::Horizontal { x1, x2, y } => {
                    for point in &mut diagram[y][x1..=x2] {
                        *point += 1;
                    }
                }
                Vent::Vertical { x, y1, y2 } => {
                    for row in &mut diagram[y1..=y2] {
                        row[x] += 1;
                    }
                }
                Vent::Diagonal { x1, x2, y1, y2 } => {
                    let dy = if y1 <= y2 { 1 } else { -1 };
                    let dx = if x1 <= x2 { 1 } else { -1 };
                    let (mut y, mut x) = (y1 as isize, x1 as isize);
                    let range = (y2 as isize - y1 as isize) * dy;
                    for _ in 0..=range {
                        diagram[y as usize][x as usize] += 1;
                        y += dy;
                        x += dx;
                    }
                }
            }
        }

        ANSWER!(count_overlapping_vents(&diagram))
    }
}

impl Day5 {
    fn parse_vents(&self) -> Result<(usize, usize, Vec<Vent>), Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut vents: Vec<Vent> = Vec::new();

        let mut width = 0;
        let mut height = 0;
        for line_res in file.lines() {
            let vent = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?
                .parse()?;

            match &vent {
                &Vent::Horizontal { x1, x2, y } => {
                    let max_x = max(x1, x2);
                    if max_x > width {
                        width = max_x;
                    }
                    if y > height {
                        height = y
                    }
                }
                &Vent::Vertical { x, y1, y2 } => {
                    let max_y = max(y1, y2);
                    if max_y > height {
                        height = max_y;
                    }
                    if x > width {
                        width = x
                    }
                }
                &Vent::Diagonal { x1, x2, y1, y2 } => {
                    let max_y = max(y1, y2);
                    if max_y > height {
                        height = max_y;
                    }
                    let max_x = max(x1, x2);
                    if max_x > width {
                        width = max_x;
                    }
                }
            }

            vents.push(vent);
        }

        Ok((width + 1, height + 1, vents))
    }
}

fn count_overlapping_vents(diagram: &Vec<Vec<usize>>) -> usize {
    diagram.iter().flatten().filter(|p| **p >= 2).count()
}
