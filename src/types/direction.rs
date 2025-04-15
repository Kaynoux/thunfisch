#[derive(Clone, Copy)]
pub enum Direction {
    Up = 8,
    Down = -8,
    Left = -1,
    Right = 1,
    UpLeft = 7,
    UpRight = 9,
    DownLeft = -9,
    DownRight = -7,
}
