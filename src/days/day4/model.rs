use std::{
    error::Error,
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

#[derive(Default)]
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

impl TryFrom<&[String]> for Board {
    type Error = Box<dyn Error>;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut this = Self::default();
        for i in 0..5 {
            this.0[i] = lines[i].as_str().try_into()?;
        }
        Ok(this)
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

#[derive(Default)]
struct BoardRow([BoardSpot; 5]);

impl TryFrom<&str> for BoardRow {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut this = Self::default();
        let mut cols = value.split_ascii_whitespace();
        for i in 0..5 {
            match cols.next().map(str::parse) {
                Some(Ok(v)) => this.0[i] = BoardSpot::new(v),
                _ => return Err(format!("invalid board row: {}", value).into()),
            }
        }
        Ok(this)
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

#[derive(Default)]
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
