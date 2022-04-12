use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::BitAnd,
    path::PathBuf,
};

use structopt::StructOpt;

use super::{todays_input, Day, PartResult, ANSWER};

todays_input!(INFILE_PATH);

const N_BITS: usize = 12;

#[derive(StructOpt)]
pub struct Day3 {
    #[structopt(default_value = &INFILE_PATH)]
    infile: PathBuf,
}

impl Day for Day3 {
    fn part1(&self) -> PartResult {
        let report = self.parse_report()?;
        let mut n_lines = 0_usize;
        let mut gamma_counts = vec![0_usize; N_BITS];

        for value in &report {
            let mut bit_index = 1;
            for index in 0..N_BITS {
                if value.bitand(bit_index) > 0 {
                    gamma_counts[index] += 1;
                }
                bit_index <<= 1;
            }

            n_lines += 1;
        }

        let mut gamma = 0_usize;
        let mut epsilon = 0_usize;
        let threshold = n_lines / 2;
        for count in gamma_counts.iter().rev() {
            gamma <<= 1;
            epsilon <<= 1;
            if *count >= threshold {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        }

        ANSWER!(gamma * epsilon)
    }

    fn part2(&self) -> PartResult {
        let mut report = self.parse_report()?;
        report.sort();

        let mut bit_index = 1_usize << (N_BITS - 1);
        let mut oxy_window = report.as_slice();
        let mut co2_window = report.as_slice();
        while bit_index > 0 && (oxy_window.len() > 1 || co2_window.len() > 1) {
            if oxy_window.len() > 1 {
                let oxy_part_i = oxy_window
                    .iter()
                    .position(|&v| v & bit_index == bit_index)
                    .unwrap_or(oxy_window.len());
                if oxy_part_i <= oxy_window.len() / 2 {
                    oxy_window = &oxy_window[oxy_part_i..];
                } else {
                    oxy_window = &oxy_window[..oxy_part_i];
                }
            }

            if co2_window.len() > 1 {
                let co2_part_i = co2_window
                    .iter()
                    .position(|&v| v & bit_index == bit_index)
                    .unwrap_or(co2_window.len());
                if co2_part_i > co2_window.len() / 2 {
                    co2_window = &co2_window[co2_part_i..];
                } else {
                    co2_window = &co2_window[..co2_part_i];
                }
            }

            bit_index >>= 1;
        }

        let oxy_answer = match oxy_window.len() {
            1 => oxy_window[0],
            _ => {
                return Err("failed to isolate one value for oxygen rate".into())
            }
        };

        let co2_answer = match co2_window.len() {
            1 => co2_window[0],
            _ => return Err("failed to isolate one value for CO2 rate".into()),
        };

        ANSWER!(oxy_answer * co2_answer)
    }
}

impl Day3 {
    fn parse_report(&self) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut report = Vec::new();

        for line_res in BufReader::new(File::open(&self.infile)?).lines() {
            let line = line_res
                .map_err(|e| format!("invalid line: {}", e).to_string())?;
            match usize::from_str_radix(&line, 2) {
                Ok(value) => report.push(value),
                _ => return Err(format!("invalid value: {}", line).into()),
            }
        }

        Ok(report)
    }
}
