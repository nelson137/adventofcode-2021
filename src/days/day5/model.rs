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
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
    Diagonal { x1: usize, x2: usize, y1: usize, y2: usize },
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
            Ok(Self::Horizontal { y: y1, x1, x2 })
        } else if x1 == x2 {
            let (y1, y2) = (y1, y2).ordered();
            Ok(Self::Vertical { x: x1, y1, y2 })
        } else {
            Ok(Self::Diagonal { x1, y1, x2, y2 })
        }
    }
}

impl fmt::Display for Vent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Vent::Horizontal { y, x1, x2 } => f.write_str(&format!(
                "Horizontal {{ ({},{}) -> ({},{}) }}",
                x1, y, x2, y
            )),
            Vent::Vertical { x, y1, y2 } => f.write_str(&format!(
                "Vertical {{ ({},{}) -> ({},{}) }}",
                x, y1, x, y2
            )),
            Vent::Diagonal { x1, x2, y1, y2 } => f.write_str(&format!(
                "Diagonal {{ ({},{}) -> ({},{}) }}",
                x1, y1, x2, y2
            )),
        }
    }
}
