use std::{
    error::Error,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Board([BoardRow; 5]);

impl Board {
    pub fn mark(&mut self, num: usize) {
        for row in &mut self.0 {
            for spot in row.iter_mut() {
                spot.mark_if_matching(num);
            }
        }
    }

    pub fn is_win(&self) -> bool {
        for row in &self.0 {
            if row.iter().all(BoardSpot::is_marked) {
                return true;
            }
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

#[derive(Debug)]
struct BoardRow([BoardSpot; 5]);

impl TryFrom<String> for BoardRow {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let row = value
            .split_whitespace()
            .map(|s| s.parse().map(BoardSpot::Unmarked))
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

#[derive(Debug)]
pub enum BoardSpot {
    Marked(usize),
    Unmarked(usize),
}

impl BoardSpot {
    fn is_marked(&self) -> bool {
        match *self {
            Self::Marked(_) => true,
            Self::Unmarked(_) => false,
        }
    }

    fn mark_if_matching(&mut self, num: usize) {
        match *self {
            Self::Unmarked(v) if v == num => *self = Self::Marked(v),
            _ => (),
        }
    }

    fn unmarked(&self) -> Option<usize> {
        match *self {
            Self::Unmarked(v) => Some(v),
            _ => None,
        }
    }
}
