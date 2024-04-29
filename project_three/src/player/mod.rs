use crate::board::percepts::PassivePercept;
use crate::board::percepts::PassivePerceptTrait;
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
	pub pos: Point,
	pub board: &'a Board,
	dir: Direction,
	arrows: u8
}

impl Player<'_> {
	pub fn new(b: &Board) -> Player {
		Player{pos: Point{x:0,y:0}, board: b, dir: Direction::East, arrows: 1u8}
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
			todo!();
		}
		else{
			println!("Sorry no arrows left");
		}
	}

	pub fn observe(&self) {
		print!("You are facing ");

		match self.dir{
			Direction::West => print!("west"),
			Direction::North => print!("north"),
			Direction::East => print!("east"),
			Direction::South => print!("south"),
		}
		println!(" in room ({}, {})", self.pos.x+1, self.pos.y+1);

		let p = self.pos.change_perspective(self.board.grid.len());

		let tile = &self.board.grid[p.x][p.y];

		let mut breeze = false;
		let mut stink = false;
		for percept in &tile.passive_percepts{
			match **percept{
				PassivePercept::Stink(_) => {
					if !stink {
						println!("{}", percept.describe());
					}
					stink = true;
				},
				PassivePercept::Breeze(_) => {
					if !breeze {
						println!("{}", percept.describe());
					}
					breeze = true;
				},
				_ => continue
			};
		}
	}
}