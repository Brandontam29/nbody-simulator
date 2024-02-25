#[derive(Debug, PartialEq)]
pub enum Quadrant {
    NW,
    NE,
    SW,
    SE,
}

impl From<&Quadrant> for usize {
    fn from(index: &Quadrant) -> Self {
        match index {
            Quadrant::NW => 0,
            Quadrant::NE => 1,
            Quadrant::SW => 2,
            Quadrant::SE => 3,
        }
    }
}
