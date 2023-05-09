use std::ops::AddAssign;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub i32, pub i32);

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Position {
    fn add(self, rhs: Self) -> Self::Output {
        Position (
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }

    type Output = Position;
}