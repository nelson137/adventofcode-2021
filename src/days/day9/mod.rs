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

        // let seed_y: usize = 95;
        // let seed_x: usize = 63;
        // let begin = std::time::SystemTime::now();
        // _fill_basin_unoptimized(&mut heights, seed_x, seed_y, 0);
        // let end = std::time::SystemTime::now();
        // let t = end.duration_since(begin)?.as_micros() as f32 / 1000.0;
        // println!("fill time = {} ms", t);

        let mut n_basins = 0;
        for y in 0..heights.len() {
            for x in 0..heights[0].len() {
                if _fill_basin_unoptimized(&mut heights, x, y, n_basins) {
                    // basin_i = (basin_i + 1) % 10;
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

        /*
        for row in &heights {
            let mut row_str = String::new();
            for h in row {
                // if y == seed_y && x == seed_x {
                //     row_str.push_str("\x1b[103m\x1b[30m");
                // }
                match *h {
                    Height::Basin(None) => row_str.push('•'),
                    // Height::Basin(Some(i)) =>
                    //     row_str.push(('0' as u8 + i as u8) as char),
                    Height::Basin(Some(_)) => row_str.push('#'),
                    _ => row_str.push(' '),
                }
                // if y == seed_y && x == seed_x {
                //     row_str.push_str("\x1b[0m");
                // }
            }
            println!("{}", row_str);
        }
        */

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
                line.chars()
                    .map(|c| c as usize - ('0' as usize))
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

fn _fill_basin_unoptimized(
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
            // x -= 1;
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

fn _fill_basin_wiki_broken(heights: &mut Vec<Vec<Height>>, x: usize, y: usize) {
    match heights[y][x] {
        Height::Basin(None) => (),
        _ => return,
    }

    let mut queue = VecDeque::from([(x, x, y, 1_isize), (x, x, y - 1, -1)]);

    while let Some((mut x1, x2, y, dy)) = queue.pop_front() {
        let mut x = x1;

        if heights[y][x] == Height::Basin(None) {
            while heights[y][x - 1] == Height::Basin(None) {
                x -= 1;
                println!("set x={},y={}", x, y);
                heights[y][x] = Height::Basin(Some(0));
            }
        }

        if x < x1 {
            queue.push_back((x, x1 - 1, (y as isize - dy) as usize, -dy));
        }

        while x1 < x2 {
            while heights[y][x1] == Height::Basin(None) {
                println!("set x={},y={}", x1, y);
                heights[y][x1] = Height::Basin(Some(0));
                x1 += 1;
            }

            queue.push_back((x, x1 - 1, (y as isize + dy) as usize, dy));
            if x1 - 1 > x2 {
                queue.push_back((
                    x2 + 1,
                    x1 - 1,
                    (y as isize - dy) as usize,
                    -dy,
                ));
            }

            while x1 < x2 && heights[y][x1] != Height::Basin(None) {
                x1 += 1;
            }

            x = x1;
        }
    }
}

/*
push(y, x, x, 1); // needed in some cases
push(y + 1, x, x, –1); // seed segment(popped 1st)
while sp > 0 do
    // pop segment off stack and fill a neighboring scan line
    pop(y, x1, x2, dy);
    // segment of scan line y – dy for x1 ≤ x ≤ x2 was previously filled,
    // now explore adjacent pixels in scan line y
    x ← x1;
    while x ≥ win.xmin and pixelread(x, y) = ov do
        pixelwrite(x, y, nv);
        x ← x – 1;
    endloop;
    if x ≥ xl then goto __SKIP;
    start ← x + 1;
    if start < xl then push(y, start, x1 – 1, – dy); // leak on left?
    x ← xl + 1;
    loop do
        while x ≤ win.xmax and pixelread(x, y) = ov do
            pixelwrite(x, y, nv);
            x ← x + 1;
        endloop;
        push(y, start, x – 1, dy);
        if x > x2 + 1 then push(y, x2 + 1, x – 1, –dy); // leak on right?
        __SKIP: x ← x + 1;
        while x ≤ x2 and pixelread(x, y) ≠ ov do
            x ← x + 1;
        endloop;
        start ← x;
    while x ≤ x2;
endloop;
*/

/*
    while let Some((y, x1, x2, dy)) = queue.pop() {
        let mut x = x1;
        while heights[y][x] == Height::Basin(None) {
            heights[y][x] = Height::Basin(Some(0));
            x -= 1;
        }
        if x >= x1 {
            goto __SKIP;
        }
        let start = x + 1;
        if start < x1 {
            queue.push((y, start, x1 - y, -dy)); // leak on left?
        }
        x = x1 + 1;
        loop {
            while heights[y][x] == Height::Basin(None) {
                heights[y][x] = Height::Basin(Some(0));
                x += 1;
            }
            queue.push((y, start, x - 1, dy));
            if x > x2 + 1 {
                queue.push((y, x2 + 1, x - 1, -dy)); // leak on right?
            }
            __SKIP: x += 1;
            while heights[y][x] != Height::Basin(None) {
                x += 1;
            }
            start = x;
            if x > x2 {
                break;
            }
        }
    }
*/

fn _fill_basin_textbook(heights: &mut Vec<Vec<Height>>, x: usize, y: usize) {
    match heights[y][x] {
        Height::Basin(None) => (),
        _ => return,
    }

    let mut queue = Vec::from([(y, x, x, 1_isize), (y + 1, x, x, -1)]);

    while let Some((y, x1, x2, dy)) = queue.pop() {
        let mut x = x1;
        while heights[y][x] == Height::Basin(None) {
            heights[y][x] = Height::Basin(Some(0));
            x -= 1;
        }
        let mut start = x + 1;
        let mut skip_iter = if x < x1 {
            if start < x1 {
                queue.push((y, start, x1 - y, -dy)); // leak on left?
            }
            x = x1 + 1;
            false
        } else {
            true
        };
        loop {
            if !skip_iter {
                skip_iter = false;
                while heights[y][x] == Height::Basin(None) {
                    heights[y][x] = Height::Basin(Some(0));
                    x += 1;
                }
                queue.push((y, start, x - 1, dy));
                if x > x2 + 1 {
                    queue.push((y, x2 + 1, x - 1, -dy)); // leak on right?
                }
            }
            x += 1;
            while heights[y][x] != Height::Basin(None) {
                x += 1;
            }
            start = x;
            if x > x2 {
                break;
            }
        }
    }
}
