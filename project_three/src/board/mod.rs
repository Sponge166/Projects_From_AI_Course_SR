#![allow(dead_code)]
use crate::board::percepts::Occupant;
use crate::board::percepts::PassivePerceptTrait;
use crate::board::percepts::OccupantTrait;
use crate::board::percepts::Tile;
use crate::board::percepts::Effect;
use crate::board::point::Point;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::rc::Rc;
use Vec;

pub mod point;
pub mod percepts;
pub mod board_analyzer;

#[derive(Debug)]
pub struct Board {
	pub grid: Vec<Vec<Tile>>
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
					let percept = occupant.get_percept_produced(); // Rc<PassivePercept>
					match percept.effect(){
						Effect::NotNeighbors => tile.add(Rc::clone(&percept)),
						Effect::Neighbors => {
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

	fn empty(p: Point) -> Board{
		let mut grid: Vec<Vec<Tile>> = Vec::new();
		for _ in 0..p.x{
			let mut temp: Vec<Tile> = Vec::new();
			for _ in 0..p.y{
				temp.push(Tile::empty());
			}
			grid.push(temp);
		}
		Board {grid}
	}

	pub fn dim(&self) -> Point {
		Point{x: self.grid.len(), y: self.grid[0].len()}
	}

	pub fn wincon(&self, p: Point) -> bool {
		if let Some(Occupant::Gold(_)) = self.grid[p.x][p.y].occupant {
			return true;
		} 
		false
	}
	pub fn wampus_losecon(&self, p: Point) -> bool {
		if let Some(Occupant::Wampus(_)) = self.grid[p.x][p.y].occupant {
			return true;
		} 
		false
	}
	pub fn pit_losecon(&self, p: Point) -> bool {
		if let Some(Occupant::Pit(_)) = self.grid[p.x][p.y].occupant {
			return true;
		} 
		false
	}
}

