use std::{
    error::Error,
    fmt::{self, Formatter},
    str::FromStr,
};

trait OrderedVars<T: PartialOrd> {
    fn ordered(self) -> (T, T);
}

impl<T: PartialOrd> OrderedVars<T> for (T, T) {
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

impl FromStr for Vent {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut points = value.split(" -> ");
        let (p1, p2) = match (points.next(), points.next(), points.next()) {
            (Some(a), Some(b), None) => (a, b),
            _ => return Err(format!("invalid vent: {}", value).into()),
        };

        let mut p1_xy = p1.split(',');
        let (x1, y1) = match (p1_xy.next(), p1_xy.next(), p1_xy.next()) {
            (Some(x), Some(y), None) => (x.trim().parse()?, y.trim().parse()?),
            _ => return Err(format!("invalid point in vent: {}", value).into()),
        };

        let mut p2_xy = p2.split(',');
        let (x2, y2) = match (p2_xy.next(), p2_xy.next(), p2_xy.next()) {
            (Some(x), Some(y), None) => (x.trim().parse()?, y.trim().parse()?),
            _ => return Err(format!("invalid point in vent: {}", value).into()),
        };

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
