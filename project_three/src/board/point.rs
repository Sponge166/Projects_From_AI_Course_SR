#[derive(Debug, Copy, Clone)]
pub struct Point{
	pub x: usize,
	pub y: usize
}

impl Point{
	pub fn change_perspective(&self, rows: usize) -> Self {
		Point {
			x: self.x.abs_diff(rows),
			y: self.y
		}
	}
}