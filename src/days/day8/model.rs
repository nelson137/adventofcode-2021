use std::{collections::HashSet, error::Error, str::FromStr};

use super::util::{set, ExpectIsolated};

#[derive(Debug)]
pub struct Entry {
    pub digits: [HashSet<char>; 10],
    pub output: [HashSet<char>; 4],
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(" | ");

        macro_rules! parse_digits {
            ($d_str:expr) => {
                $d_str
                    .trim()
                    .split_whitespace()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .collect::<Vec<_>>()
            };
        }

        let digits = match components.next() {
            Some(d) => parse_digits!(d).try_into().map_err(|_| {
                format!("invalid number of digits in entry: {}", s).to_string()
            })?,
            None => return Err(format!("no digits in entry: {}", s).into()),
        };

        let output = match components.next() {
            Some(d) => parse_digits!(d).try_into().map_err(|_| {
                format!("invalid number of output digits in entry: {}", s)
                    .to_string()
            })?,
            None => return Err(format!("no output in entry: {}", s).into()),
        };

        if components.next().is_some() {
            return Err(format!("invalid entry: {}", s).into());
        }

        Ok(Self { digits, output })
    }
}

impl Entry {
    pub fn solve(&self) -> usize {
        let digit1 = self
            .digits
            .iter()
            .find(|d| d.len() == 2)
            .expect("failed to find digit 1");

        let mut maybe_c = None;
        for digit in self.digits.iter().filter(|n| n.len() == 6) {
            let diff = digit1.difference(digit).collect::<Vec<_>>();
            if diff.len() == 1 {
                maybe_c = Some(**diff.iter().expect_isolated());
                break;
            }
        }
        let c = maybe_c.expect("failed to isolate c");

        let f = *digit1.difference(&set![c]).expect_isolated();

        let mut maybe_digit2 = None;
        let mut maybe_digit3 = None;
        for d in self.digits.iter().filter(|d| d.len() == 5) {
            if d.contains(&c) {
                if d.contains(&f) {
                    maybe_digit3 = Some(d);
                } else {
                    maybe_digit2 = Some(d);
                }
            }
        }
        let digit2 = maybe_digit2.expect("failed to isolate digit 2");
        let digit3 = maybe_digit3.expect("failed to isolate digit 3");

        let e = *digit2.difference(&digit3).expect_isolated();

        let mut answer = 0;
        for output in &self.output {
            answer *= 10;
            answer += match output.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                // digits with length 5 have unique segments:
                //   2 => c, e
                //   3 => c, f
                //   5 => b, f
                5 =>
                    if output.contains(&c) {
                        if output.contains(&f) {
                            3
                        } else {
                            2
                        }
                    } else {
                        5
                    },
                // digits with length 6 have unique segments:
                //   0 => c, e
                //   6 => d, e
                //   9 => c, d
                6 =>
                    if output.contains(&c) {
                        if output.contains(&e) {
                            0
                        } else {
                            9
                        }
                    } else {
                        6
                    },
                _ => panic!("invalid output digit: {:?}", output),
            };
        }

        answer
    }
}
