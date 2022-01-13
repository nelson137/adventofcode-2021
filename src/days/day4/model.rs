use std::{
    error::Error,
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

pub struct Board([BoardRow; 5]);

impl Board {
    pub fn mark_check(&mut self, num: usize) -> bool {
        let mut is_win = false;
        for row in &mut self.0 {
            let mut row_is_win = true;
            for spot in row.iter_mut() {
                row_is_win &= spot.mark_if_matching(num);
            }
            is_win |= row_is_win;
        }
        if is_win {
            return true;
        }

        for spot_i in 0..5 {
            if self.0.iter().all(|r| r[spot_i].is_marked()) {
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter_map(BoardSpot::unmarked)
            .sum()
    }
}

impl TryFrom<Vec<String>> for Board {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|v: Vec<BoardRow>| {
                    format!("invalid number of rows for board: {}", v.len())
                        .to_string()
                })?,
        ))
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            f.write_fmt(format_args!("{:?}", row))?;
        }
        Ok(())
    }
}

struct BoardRow([BoardSpot; 5]);

impl TryFrom<String> for BoardRow {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let row = value
            .split_whitespace()
            .map(|s| s.parse().map(BoardSpot::new))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("invalid row: {}", value).to_string())?;
        Ok(Self(row.try_into().map_err(|v: Vec<BoardSpot>| {
            format!("invalid number of values for board row: {}", v.len())
                .to_string()
        })?))
    }
}

impl Deref for BoardRow {
    type Target = [BoardSpot; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoardRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for BoardRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for spot in &self.0 {
            f.write_fmt(format_args!("{:?} ", spot))?;
        }
        println!("");
        Ok(())
    }
}

struct BoardSpot {
    value: usize,
    marked: bool,
}

impl BoardSpot {
    #[inline]
    fn new(value: usize) -> Self {
        Self { value, marked: false }
    }

    #[inline]
    fn is_marked(&self) -> bool {
        self.marked
    }

    #[inline]
    fn mark_if_matching(&mut self, num: usize) -> bool {
        if self.value == num {
            self.marked = true;
            true
        } else {
            self.marked
        }
    }

    #[inline]
    fn unmarked(&self) -> Option<usize> {
        if self.marked {
            None
        } else {
            Some(self.value)
        }
    }
}

impl Debug for BoardSpot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.marked {
            f.write_str("\x1b[33m")?;
        }
        f.write_fmt(format_args!("{:2}", self.value))?;
        if self.marked {
            f.write_str("\x1b[0m")?;
        }
        Ok(())
    }
}
