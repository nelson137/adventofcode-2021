use std::{
    cmp::max,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::Vent;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day5 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day5 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let (width, height, vents) = self.parse_vents()?;

        let mut diagram: Vec<Vec<usize>> = vec![vec![0_usize; width]; height];

        for ls in &vents {
            match ls {
                Vent::Horizontal(lsh) => {
                    for point in &mut diagram[lsh.y][lsh.x1..=lsh.x2] {
                        *point += 1;
                    }
                }
                Vent::Vertical(lsv) => {
                    for row in &mut diagram[lsv.y1..=lsv.y2] {
                        row[lsv.x] += 1;
                    }
                }
                Vent::Diagonal(_) => (),
            }
        }

        let answer = diagram.iter().flatten().filter(|p| **p >= 2).count();
        println!("{}", answer);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let (width, height, vents) = self.parse_vents()?;

        let mut diagram: Vec<Vec<usize>> = vec![vec![0_usize; width]; height];

        for ls in &vents {
            match ls {
                Vent::Horizontal(lsh) => {
                    for point in &mut diagram[lsh.y][lsh.x1..=lsh.x2] {
                        *point += 1;
                    }
                }
                Vent::Vertical(lsv) => {
                    for row in &mut diagram[lsv.y1..=lsv.y2] {
                        row[lsv.x] += 1;
                    }
                }
                Vent::Diagonal(lsd) => {
                    let dy = if lsd.y1 <= lsd.y2 { 1 } else { -1 };
                    let dx = if lsd.x1 <= lsd.x2 { 1 } else { -1 };
                    let (mut y, mut x) = (lsd.y1 as isize, lsd.x1 as isize);
                    let range = (lsd.y2 as isize - lsd.y1 as isize) * dy;
                    for _ in 0..=range {
                        diagram[y as usize][x as usize] += 1;
                        y += dy;
                        x += dx;
                    }
                }
            }
        }

        let answer = diagram.iter().flatten().filter(|p| **p >= 2).count();
        println!("{}", answer);

        Ok(())
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
                Vent::Horizontal(v_h) => {
                    let max_x = max(v_h.x1, v_h.x2);
                    if max_x > width {
                        width = max_x;
                    }
                    if v_h.y > height {
                        height = v_h.y
                    }
                }
                Vent::Vertical(v_v) => {
                    let max_y = max(v_v.y1, v_v.y2);
                    if max_y > height {
                        height = max_y;
                    }
                    if v_v.x > width {
                        width = v_v.x
                    }
                }
                Vent::Diagonal(v_d) => {
                    let max_y = max(v_d.y1, v_d.y2);
                    if max_y > height {
                        height = max_y;
                    }
                    let max_x = max(v_d.x1, v_d.x2);
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
