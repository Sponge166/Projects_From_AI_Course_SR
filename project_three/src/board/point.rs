#[derive(Debug, Copy, Clone, Eq)]
#[derive(PartialEq)]
pub struct Point{
	pub x: usize,
	pub y: usize
}

impl Point{
	pub fn board_perspective(&self, rows: usize) -> Self {
		Point {
			x: self.y.abs_diff(rows-1),
			y: self.x
		}
	}

	pub fn player_perspective(&self, rows: usize) -> Self {
		Point {
			x: self.y,
			y: self.x.abs_diff(rows-1)
		}
	}
}