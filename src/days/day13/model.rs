use std::{
    error::Error,
    fmt::{self, Debug, Display, Write},
    str::FromStr,
};

pub struct Paper {
    paper: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Paper {
    pub fn from_coords(
        coords: Vec<(usize, usize)>,
    ) -> Result<Self, Box<dyn Error>> {
        let (mut width, mut height) = coords
            .iter()
            .fold((0, 0), |acc, c| (acc.0.max(c.0), acc.1.max(c.1)));
        width += 1;
        height += 1;

        let mut paper = vec![vec![false; width]; height];
        for (x, y) in coords {
            paper[y][x] = true;
        }

        Ok(Self { paper, width, height })
    }

    pub fn fold(&mut self, ins: &Instruction) {
        match *ins {
            Instruction::Up(fold) => {
                for y in (fold + 1)..self.height {
                    for x in 0..self.width {
                        self.paper[fold - (y - fold)][x] |= self.paper[y][x];
                    }
                }
                self.height = fold;
            }
            Instruction::Left(fold) => {
                for y in 0..self.height {
                    for x in (fold + 1)..self.width {
                        self.paper[y][fold - (x - fold)] |= self.paper[y][x];
                    }
                }
                self.width = fold;
            }
        }
    }

    pub fn count_dots(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.paper[y][x] {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Debug for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            let row_str: String = self.paper[y]
                .iter()
                .take(self.width)
                .map(|p| if *p { '#' } else { '.' })
                .collect();
            f.write_str(&row_str)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            let row_str: String = self.paper[y]
                .iter()
                .take(self.width)
                .map(|p| if *p { '#' } else { ' ' })
                .collect();
            f.write_str(&row_str)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {
    Up(usize),
    Left(usize),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! invalid_ins {
            () => {
                format!("invalid instruction: {}", s)
            };
        }

        let location = if s.starts_with("fold along ") {
            &s[11..]
        } else {
            return Err(invalid_ins!().into());
        };

        let mut axis_n = location.split('=');

        let (axis, n) = match (axis_n.next(), axis_n.next(), axis_n.next()) {
            (Some(axis), Some(n), None) => (axis.trim(), n.trim()),
            _ => return Err(invalid_ins!().into()),
        };

        let value = n.parse().map_err(|_| invalid_ins!().to_string())?;

        if axis == "x" {
            Ok(Self::Left(value))
        } else if axis == "y" {
            Ok(Self::Up(value))
        } else {
            Err(invalid_ins!().into())
        }
    }
}
