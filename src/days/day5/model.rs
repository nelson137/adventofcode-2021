use std::{
    error::Error,
    fmt::{self, Formatter},
};

use regex::Captures;

trait OrderedVars<T>
where
    T: PartialOrd,
{
    fn ordered(self) -> (T, T);
}

impl<T> OrderedVars<T> for (T, T)
where
    T: PartialOrd,
{
    #[inline]
    fn ordered(self) -> (T, T) {
        if self.0 <= self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }
}

#[derive(Debug)]
pub enum Vent {
    Horizontal(VentHorizontal),
    Vertical(VentVertical),
    Diagonal(VentDiagonal),
}

impl<'a> TryFrom<Captures<'a>> for Vent {
    type Error = Box<dyn Error>;

    fn try_from(cap: Captures) -> Result<Self, Self::Error> {
        macro_rules! parse_coord_match {
            ($i:literal) => {{
                cap.get($i).unwrap().as_str().parse()?
            }};
        }
        let x1 = parse_coord_match!(1);
        let y1 = parse_coord_match!(2);
        let x2 = parse_coord_match!(3);
        let y2 = parse_coord_match!(4);
        if y1 == y2 {
            let (x1, x2) = (x1, x2).ordered();
            Ok(Self::Horizontal(VentHorizontal { y: y1, x1, x2 }))
        } else if x1 == x2 {
            let (y1, y2) = (y1, y2).ordered();
            Ok(Self::Vertical(VentVertical { x: x1, y1, y2 }))
        } else {
            Ok(Self::Diagonal(VentDiagonal { x1, y1, x2, y2 }))
        }
    }
}

#[derive(Debug)]
pub struct VentHorizontal {
    pub y: usize,
    pub x1: usize,
    pub x2: usize,
}

impl fmt::Display for VentHorizontal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!(
            "VentHorizontal {{ ({},{}) -> ({},{}) }}",
            self.x1, self.y, self.x2, self.y
        ))
    }
}

#[derive(Debug)]
pub struct VentVertical {
    pub x: usize,
    pub y1: usize,
    pub y2: usize,
}

impl fmt::Display for VentVertical {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!(
            "VentVertical {{ ({},{}) -> ({},{}) }}",
            self.x, self.y1, self.x, self.y2
        ))
    }
}

#[derive(Debug)]
pub struct VentDiagonal {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl fmt::Display for VentDiagonal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!(
            "VentDiagonal {{ ({},{}) -> ({},{}) }}",
            self.x1, self.y1, self.x2, self.y2
        ))
    }
}
