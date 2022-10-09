#[allow(dead_code)]
#[derive(Debug)]
pub enum Rotation {
    R,
    L,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
impl Direction {
    fn get(&self, r: Rotation) -> Self {
        use Direction::{East as E, North as N, South as S, West as W};
        match self {
            N => match r {
                Rotation::R => E,
                Rotation::L => W,
            },
            E => match r {
                Rotation::R => S,
                Rotation::L => N,
            },
            W => match r {
                Rotation::R => N,
                Rotation::L => S,
            },
            S => match r {
                Rotation::R => W,
                Rotation::L => E,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

#[allow(dead_code)]
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot {
            d: self.d.get(Rotation::R),
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot {
            d: self.d.get(Rotation::L),
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        use Direction::{East as E, North as N, South as S, West as W};
        let Robot { x, y, d } = self;
        let (x_, y_): (i32, i32) = match d {
            N => (x, y + 1),
            E => (x + 1, y),
            W => (x - 1, y),
            S => (x, y - 1),
        };

        Robot { x: x_, y: y_, d }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, ch| match ch {
            'R' => r.turn_right(),
            'L' => r.turn_left(),
            _ => r.advance(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
