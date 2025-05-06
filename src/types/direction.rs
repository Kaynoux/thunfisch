#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
}

impl Dir {
    /// Increment
    pub const fn inc(&mut self) {
        *self = Dir::new(*self as usize + 1)
    }

    pub const fn new(idx: usize) -> Dir {
        match idx {
            0 => Dir::Up,
            1 => Dir::UpRight,
            2 => Dir::Right,
            3 => Dir::DownRight,
            4 => Dir::Down,
            5 => Dir::DownLeft,
            6 => Dir::Left,
            7 => Dir::UpLeft,
            _ => panic!("Direction out of bounds"),
        }
    }
}
