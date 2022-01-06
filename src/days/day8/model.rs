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

        let digits = match components.next() {
            Some(d) => d
                .trim()
                .split_whitespace()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| {
                    format!("invalid number of digits in entry: {}", s)
                        .to_string()
                })?,
            None => return Err(format!("no digits in entry: {}", s).into()),
        };

        let output = match components.next() {
            Some(d) => d
                .trim()
                .split_whitespace()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| {
                    format!("invalid number of output digits in entry: {}", s)
                        .to_string()
                })?,
            None => return Err(format!("no output in entry: {}", s).into()),
        };

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
        let digit4 = self
            .digits
            .iter()
            .find(|d| d.len() == 4)
            .expect("failed to find digit 4");
        let digit7 = self
            .digits
            .iter()
            .find(|d| d.len() == 3)
            .expect("failed to find digit 7");

        let a = *digit7.difference(&digit1).expect_isolated();

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
            if d.contains(&c) && d.contains(&f) {
                maybe_digit3 = Some(d);
            } else if d.contains(&c) {
                maybe_digit2 = Some(d);
            }
        }
        let digit2 = maybe_digit2.expect("failed to isolate digit 2");
        let digit3 = maybe_digit3.expect("failed to isolate digit 3");

        let e = *digit2.difference(&digit3).expect_isolated();

        let b = *digit4.difference(&digit3).expect_isolated();

        let d = **digit4
            .difference(&digit1)
            .collect::<HashSet<_>>()
            .difference(&set![&b])
            .expect_isolated();

        let g = **digit3
            .difference(&digit7)
            .collect::<HashSet<_>>()
            .difference(&set![&d])
            .expect_isolated();

        // println!(" {0}{0}{0}{0}", a);
        // println!("{}    {}", b, c);
        // println!("{}    {}", b, c);
        // println!(" {0}{0}{0}{0}", d);
        // println!("{}    {}", e, f);
        // println!("{}    {}", e, f);
        // println!(" {0}{0}{0}{0}", g);

        let output_digits = vec![
            set![a, b, c, e, f, g],
            set![c, f],
            set![a, c, d, e, g],
            set![a, c, d, f, g],
            set![b, c, d, f],
            set![a, b, d, f, g],
            set![a, b, d, e, f, g],
            set![a, c, f],
            set![a, b, c, d, e, f, g],
            set![a, b, c, d, f, g],
        ];

        let mut answer = 0;
        for output in &self.output {
            answer *= 10;
            answer += output_digits
                .iter()
                .position(|od| od == output)
                .unwrap_or_else(|| {
                    panic!("invalid output digit: {:?}", output)
                });
        }

        answer
    }
}
