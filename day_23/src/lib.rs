use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}
impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match *self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
            Direction::NW => (-1, -1),
            Direction::NE => (-1, 1),
            Direction::SW => (1, -1),
            Direction::SE => (1, 1),
        }
    }
}

#[derive(Copy, PartialEq, Clone)]
pub enum DirectionGroup {
    N,
    S,
    E,
    W,
}

impl DirectionGroup {
    pub fn group(&self) -> Vec<Direction> {
        match *self {
            DirectionGroup::N => vec![Direction::N, Direction::NE, Direction::NW],
            DirectionGroup::S => vec![Direction::S, Direction::SE, Direction::SW],
            DirectionGroup::E => vec![Direction::E, Direction::SE, Direction::NE],
            DirectionGroup::W => vec![Direction::W, Direction::SW, Direction::NW],
        }
    }

    pub fn direction(&self) -> Direction {
        match &self {
            DirectionGroup::N => Direction::N,
            DirectionGroup::S => Direction::S,
            DirectionGroup::E => Direction::E,
            DirectionGroup::W => Direction::W,
        }
    }

    pub fn turn(&self) -> DirectionGroup {
        match *self {
            DirectionGroup::N => DirectionGroup::S,
            DirectionGroup::S => DirectionGroup::W,
            DirectionGroup::W => DirectionGroup::E,
            DirectionGroup::E => DirectionGroup::N,
        }
    }
}

pub struct Elf {
    pub index: i32,
    pub position: (i32, i32),
}
