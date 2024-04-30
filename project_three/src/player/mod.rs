use crate::board::percepts::Occupant;
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
	pub fn new(b: & Board) -> Player {
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
			
			match self.dir {
				Direction::South => self.shoot_dir(|p: &mut Point| p.x+=1),
				Direction::North => self.shoot_dir(|p: &mut Point| if p.x != 0 {p.x-=1}),
				Direction::East => self.shoot_dir(|p: &mut Point| p.y+=1),
				Direction::West => self.shoot_dir(|p: &mut Point| if p.y != 0 {p.y-=1})
			};
		}
		else{
			println!("Sorry no arrows left");
		}
	}

	fn shoot_dir<F:FnMut(&mut Point)>(&self, mut func: F) {
		let mut p_copy = self.pos.board_perspective(self.board.grid.len());
		while p_copy.x < self.board.grid.len() && p_copy.y < self.board.grid.len(){
			// while in bounds check if wampus is in this tile
			let tile = & self.board.grid[p_copy.x][p_copy.y];
			if let Some(Occupant::Wampus(wamp)) = & tile.occupant{
				wamp.frag_it();
				println!("ARRGH the wampus screams out in pain. The cave is now safe from the Wampus.");
				return;
			}

			match self.dir {
				Direction::West if p_copy.y == 0 => break,
				Direction::North if p_copy.x == 0 => break,
				_ => ()
			}
			func(&mut p_copy);
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

		let p = self.pos.board_perspective(self.board.grid.len());

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