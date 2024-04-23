use crate::board::Board;
use crate::board::point::Point;

#[derive(Debug)]
enum Direction {
	North,
	East,
	South,
	West
}

#[derive(Debug)]
pub struct Player<'a> {
	pos: Point,
	board: &'a Board,
	dir: Direction,
	arrows: u8
}

impl Player<'_> {
	pub fn new(b: &Board) -> Player {
		Player{pos: Point{x:1,y:1}, board: b, dir: Direction::East, arrows: 1u8}
	}

	pub fn turn_left(&mut self) {
		self.dir = match self.dir {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
		};
	}

	pub fn turn_right(&mut self) {
		self.dir = match self.dir {
			Direction::West => Direction::North,
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
		};
	}

	pub fn forward(&mut self) {
		match self.dir {
			Direction::West => {
				if self.pos.x > 0 {
					self.pos.x -= 1;
				}
				else{
					println!("BONK you hit a wall");
				}
			},
			Direction::North => {
				if self.board.grid[0].len() -1 > self.pos.y {
					self.pos.y += 1;
				}
				else{
					println!("BONK you hit a wall");
				}
			},
			Direction::East => {
				if self.board.grid.len() -1 > self.pos.x {
					self.pos.x += 1;
				}
				else{
					println!("BONK you hit a wall");
				}
			},
			Direction::South =>{
				if self.pos.y > 0 {
					self.pos.y -= 1;
				}
				else{
					println!("BONK you hit a wall");
				}
			},
		};
	}

	pub fn shoot(&mut self) { 
		if self.arrows > 0{
			self.arrows -= 1;
		}
		else{
			println!("Sorry no arrows left");
		}
	}
}