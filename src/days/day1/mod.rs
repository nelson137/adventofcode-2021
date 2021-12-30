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
pub struct Day1 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day1 {
    fn part1(&self) -> Result<(), Box<dyn Error>> {
        let answer = self.calc_depth_increase(1)?;
        println!("{}", answer);
        Ok(())
    }

    fn part2(&self) -> Result<(), Box<dyn Error>> {
        let answer = self.calc_depth_increase(3)?;
        println!("{}", answer);
        Ok(())
    }
}

impl Day1 {
    fn calc_depth_increase(
        &self,
        window_size: usize,
    ) -> Result<usize, Box<dyn Error>> {
        let file = BufReader::new(File::open(&self.infile)?);

        let mut window = vec![0_isize; window_size];
        let mut window_len = 0;
        let mut subwin_persist = vec![0_isize; window_size - 1];
        let mut prev_sum = 0;
        let mut answer = 0_usize;

        for line_res in file.lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;

            let depth = line
                .parse()
                .map_err(|_| format!("invalid depth: {}", line).to_string())?;

            if window_len < window_size {
                window[window_len] = depth;
                window_len += 1;
                if window_len == window_size {
                    prev_sum = window.iter().sum();
                }
                continue;
            }

            subwin_persist.copy_from_slice(&window[1..]);
            window[..window_len - 1].copy_from_slice(&subwin_persist);
            window[window_len - 1] = depth;

            let sum = window.iter().sum();
            if sum > prev_sum {
                answer += 1;
            }
            prev_sum = sum;
        }

        if window_len < window_size {
            Err("not enough data".into())
        } else {
            Ok(answer)
        }
    }
}
