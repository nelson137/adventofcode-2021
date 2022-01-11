use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Debug},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::util::ToNeighborhood;

pub struct Octopuses {
    grid: Vec<Vec<u32>>,
    simulation_queue: VecDeque<(usize, usize)>,
}

impl Octopuses {
    pub fn from(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = BufReader::new(File::open(path.as_ref())?);

        let mut grid = Vec::new();

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            grid.push(
                line.chars().map(|n| n as u32 - '0' as u32).collect::<Vec<_>>(),
            );
        }

        Ok(Self { grid, simulation_queue: VecDeque::new() })
    }

    pub fn len(&self) -> usize {
        self.grid.iter().map(Vec::len).sum()
    }

    pub fn simulate_step(&mut self) -> usize {
        self.simulation_queue
            .extend((0..10).map(|y| (0..10).map(move |x| (x, y))).flatten());

        while let Some((x, y)) = self.simulation_queue.pop_front() {
            let mut o = self.grid[y][x];
            o += 1;
            if o == 10 {
                self.simulation_queue.extend((x, y).to_neighborhood());
            }
            self.grid[y][x] = o;
        }

        let mut flashes: usize = 0;
        for o in self.grid.iter_mut().flatten() {
            if *o > 9 {
                flashes += 1;
                *o = 0;
            }
        }

        flashes
    }
}

impl Iterator for Octopuses {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.simulate_step())
    }
}

impl Debug for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for o in row {
                if *o == 0 {
                    f.write_str("\x1b[92m")?;
                }
                f.write_fmt(format_args!("{:2} ", o))?;
                if *o == 0 {
                    f.write_str("\x1b[0m")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
