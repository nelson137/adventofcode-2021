use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day};

mod model;
use self::model::Height;

todays_input!(INFILE_PATH);

#[derive(StructOpt)]
pub struct Day9 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day9 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let heights = self.parse_height_map()?;
        let max_i = heights.len() - 1;
        let max_j = heights[0].len() - 1;

        let mut local_min_sums = 0;

        // Check first row
        if heights[0][0] < heights[0][1] && heights[0][0] < heights[1][0] {
            local_min_sums += heights[0][0] + 1;
        }
        for j in 1..max_j {
            if heights[0][j] < heights[0][j - 1]
                && heights[0][j] < heights[0][j + 1]
                && heights[0][j] < heights[1][j]
            {
                local_min_sums += heights[0][j] + 1;
            }
        }
        if heights[0][max_j] < heights[0][max_j - 1]
            && heights[0][max_j] < heights[1][max_j]
        {
            local_min_sums += heights[0][max_j] + 1;
        }

        // Check middle rows
        for i in 1..max_i {
            if heights[i][0] < heights[i][1]
                && heights[i][0] < heights[i - 1][0]
                && heights[i][0] < heights[i + 1][0]
            {
                local_min_sums += heights[i][0] + 1;
            }
            for j in 1..max_j {
                if heights[i][j] < heights[i][j - 1]
                    && heights[i][j] < heights[i][j + 1]
                    && heights[i][j] < heights[i - 1][j]
                    && heights[i][j] < heights[i + 1][j]
                {
                    local_min_sums += heights[i][j] + 1;
                }
            }
            if heights[i][max_j] < heights[i][max_j - 1]
                && heights[i][max_j] < heights[i - 1][max_j]
                && heights[i][max_j] < heights[i + 1][max_j]
            {
                local_min_sums += heights[i][max_j] + 1;
            }
        }

        // Check last row
        if heights[max_i][0] < heights[max_i][1]
            && heights[max_i][0] < heights[max_i - 1][0]
        {
            local_min_sums += heights[max_i][0] + 1;
        }
        for j in 1..max_j {
            if heights[max_i][j] < heights[max_i][j - 1]
                && heights[max_i][j] < heights[max_i][j + 1]
                && heights[max_i][j] < heights[max_i - 1][j]
            {
                local_min_sums += heights[max_i][j] + 1;
            }
        }
        if heights[max_i][max_j] < heights[max_i][max_j - 1]
            && heights[max_i][max_j] < heights[max_i - 1][max_j]
        {
            local_min_sums += heights[max_i][max_j] + 1;
        }

        println!("{}", local_min_sums);

        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let mut heights = self.parse_basin_map()?;

        let mut n_basins = 0;
        for y in 0..heights.len() {
            for x in 0..heights[0].len() {
                if fill_basin(&mut heights, x, y, n_basins) {
                    n_basins += 1;
                }
            }
        }

        if n_basins < 3 {
            return Err("Not enough basins".into());
        }

        let mut basin_sizes = vec![0_usize; n_basins];
        for h in heights.iter().flatten() {
            if let Height::Basin(Some(i)) = h {
                basin_sizes[*i] += 1;
            }
        }

        basin_sizes.sort();
        let answer = basin_sizes.iter().rev().take(3).fold(1, |acc, s| acc * s);
        println!("{}", answer);

        Ok(())
    }
}

impl Day9 {
    fn parse_height_map(&self) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut heights = Vec::new();

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            heights.push(
                line.bytes()
                    .map(|b| b as usize - '0' as usize)
                    .collect::<Vec<_>>(),
            );
        }

        Ok(heights)
    }

    fn parse_basin_map(&self) -> Result<Vec<Vec<Height>>, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut heights = Vec::new();

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            heights.push(line.chars().map(Into::into).collect());
        }

        Ok(heights)
    }
}

fn fill_basin(
    heights: &mut Vec<Vec<Height>>,
    x: usize,
    y: usize,
    basin_i: usize,
) -> bool {
    match heights[y][x] {
        Height::Basin(None) => (),
        _ => return false,
    }

    let max_y = heights.len() - 1;
    let max_x = heights[0].len() - 1;
    let mut queue = VecDeque::from([(x, y)]);

    macro_rules! scan {
        ($lx:expr, $rx:expr, $y:expr) => {{
            let mut added = false;
            for x in $lx..$rx {
                if heights[$y][x] != Height::Basin(None) {
                    added = false;
                } else if !added {
                    queue.push_back((x, $y));
                    added = true;
                }
            }
        }};
    }

    while let Some((mut x, y)) = queue.pop_front() {
        let mut lx = x;
        while lx > 0 && heights[y][lx - 1] == Height::Basin(None) {
            lx -= 1;
            heights[y][lx] = Height::Basin(Some(basin_i));
        }
        while x <= max_x && heights[y][x] == Height::Basin(None) {
            heights[y][x] = Height::Basin(Some(basin_i));
            x += 1;
        }
        if x > 0 {
            if y < max_y {
                scan!(lx, x, y + 1);
            }
            if y > 0 {
                scan!(lx, x, y - 1);
            }
        }
    }

    true
}
