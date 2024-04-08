#![allow(dead_code)]
use std::rc::Rc;
use Vec;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub mod board_analyzer;

#[derive(Debug)]
pub struct Board {
	grid: Vec<Vec<Tile>>
}

impl Board{
	pub fn from_file(file_name: &str) -> std::io::Result<Board>{
		let mut grid: Vec<Vec<Tile>> = Vec::new();
		let file = File::open(file_name)?;
		let reader = BufReader::new(file);

		for line in reader.lines(){
			let mut temp: Vec<Tile> = Vec::new();
			let line = line?;

			for c in line.split(","){
				temp.push(
					match Tile::from_char(c) {
						Some(x) => x,
						None => Tile::empty()
					} 
				);
			}

			grid.push(temp);
		}

		let mut out = Board{grid};
		out.propogate_percepts();
		Ok(out)
	}

	fn propogate_percepts(&mut self){

		// if the occupant of self.grid[i][j] effects its neighbors this function

		for i in 0..self.grid.len(){
			for j in 0..self.grid[0].len(){
				let tile = &mut self.grid[i][j];
				if let Some(occupant) = &tile.occupant{
					let percept = occupant.get_percept_produced(); // Rc<dyn PassivePercept>
					match percept.effect(){
						Effect::NotNeighbors => tile.add(Rc::clone(&percept)),
						Effect:: Neighbors => {
							if i > 0{
								self.grid[i-1][j].add(Rc::clone(&percept));
							}
							if j > 0{
								self.grid[i][j-1].add(Rc::clone(&percept));
							}
							if i < self.grid.len()-1{
								self.grid[i+1][j].add(Rc::clone(&percept));
							}
							if j < self.grid[0].len()-1{
								self.grid[i][j+1].add(Rc::clone(&percept));
							}
						}
					}
				}
			}
		}
	}

	fn empty(x: usize, y: usize) -> Board{
		let mut grid: Vec<Vec<Tile>> = Vec::new();
		for _ in 0..x{
			let mut temp: Vec<Tile> = Vec::new();
			for _ in 0..y{
				temp.push(Tile::empty());
			}
			grid.push(temp);
		}
		Board {grid}
	}
}


struct Tile{
	occupant: Option<Box<dyn Occupant>>,
	passive_percepts: Vec<Rc<dyn PassivePercept>>,
	occupant_str: String
}

impl std::fmt::Debug for Tile{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
		write!(f, "{}", self.occupant_str)
	}
}


impl Tile{
	fn empty() -> Tile{
		Tile {occupant: None, passive_percepts: Vec::new(), occupant_str: String::from("X")}
	}

	fn from_char(c: &str) -> Option<Tile>{
		Some(Tile { occupant: match c {
			"P" => Some(Box::new(Pit(Breeze))),
			"G" => Some(Box::new(Gold(Glitter))),
			"W" => Some(Box::new(Wampus(Stink))),
			_ => None
		}, passive_percepts: Vec::new(),
		occupant_str: String::from(c)})
	}

	fn add(&mut self, percept: Rc<dyn PassivePercept>){
		self.passive_percepts.push(percept);
	}
}

trait Occupant {
	fn get_percept_produced(&self) -> Rc<dyn PassivePercept>;
}

trait PassivePercept: std::fmt::Debug{
	fn describe(&self) -> String;
	fn effect(&self) -> Effect;
}

enum Effect{
	Neighbors,
	NotNeighbors
}


// percept and occupant defs


#[derive(Debug, Copy, Clone)]
struct Stink;

impl PassivePercept for Stink{
	fn describe(&self) -> String{
		String::from("You smell something foul")
	}

	fn effect(&self) -> Effect{
		Effect::Neighbors
	}
}

#[derive(Debug)]
struct Wampus(Stink);

impl Occupant for Wampus{
	fn get_percept_produced(&self) -> Rc<dyn PassivePercept>{
		Rc::new(self.0)
	}
}


#[derive(Debug, Copy, Clone)]
struct Glitter;

impl PassivePercept for Glitter{
	fn describe(&self) -> String{
		String::from("You see a bright Gitter! You've found the Gold")
	}

	fn effect(&self) -> Effect{
		Effect::NotNeighbors
	}
}

#[derive(Debug)]
struct Gold(Glitter);

impl Occupant for Gold{
	fn get_percept_produced(&self) -> Rc<dyn PassivePercept>{
		Rc::new(self.0)
	}
}


#[derive(Debug, Copy, Clone)]
struct Breeze;

impl PassivePercept for Breeze{
	fn describe(&self) -> String{
		String::from("You feel a strong Breeze ")
	}

	fn effect(&self) -> Effect{
		Effect::Neighbors
	}
}

#[derive(Debug)]
struct Pit(Breeze);

impl Occupant for Pit{
	fn get_percept_produced(&self) -> Rc<dyn PassivePercept>{
		Rc::new(self.0)
	}
}
