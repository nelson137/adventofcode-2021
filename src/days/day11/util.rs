#[derive(Debug)]
pub struct Neighborhood {
    coords: Vec<(usize, usize)>,
    i: usize,
}

pub trait ToNeighborhood {
    fn to_neighborhood(self) -> Neighborhood;
}

impl ToNeighborhood for (usize, usize) {
    fn to_neighborhood(self) -> Neighborhood {
        let (x, y) = self;
        // Row above
        let mut coords = Vec::with_capacity(8);
        if y > 0 {
            let n_y = y - 1;
            if x > 0 {
                coords.push((x - 1, n_y));
            }
            coords.push((x, n_y));
            if x < 9 {
                coords.push((x + 1, n_y))
            }
        }
        // Current row, left & right
        if x > 0 {
            coords.push((x - 1, y));
        }
        if x < 9 {
            coords.push((x + 1, y));
        }
        // Row below
        if y < 9 {
            let n_y = y + 1;
            if x > 0 {
                coords.push((x - 1, n_y));
            }
            coords.push((x, n_y));
            if x < 9 {
                coords.push((x + 1, n_y))
            }
        }
        Neighborhood { coords, i: 0 }
    }
}

impl Iterator for Neighborhood {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.coords.len() {
            let i = self.i;
            self.i += 1;
            Some(self.coords[i])
        } else {
            None
        }
    }
}
