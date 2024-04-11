#![allow(dead_code)]
use crate::board::{Board};
use Vec;
use crate::board::point::Point;

struct InternalBoard{
	grid: Vec<Vec<InternalTile>>
}

impl InternalBoard{
	fn empty(p: Point) -> InternalBoard{
		let mut grid: Vec<Vec<InternalTile>> = Vec::new();
		for _ in 0..p.x{
			let mut temp: Vec<InternalTile> = Vec::new();
			for _ in 0..p.y{
				temp.push(InternalTile::empty());
			}
			grid.push(temp);
		}
		InternalBoard {grid}
	}

	fn neighbors(&self, p: Point) -> Vec<&mut InternalTile>{
		let mut out = Vec::new();
		let d = self.dim();

		if p.x > 0 {
			out.push(&mut self.grid[p.x-1][p.y]);
		}
		if p.y > 0 {
			out.push(&mut self.grid[p.x][p.y-1]);
		}
		if p.x < d.x -1{
			out.push(&mut self.grid[p.x+1][p.y]);
		}
		if p.y < d.x -1 {
			out.push(&mut self.grid[p.x][p.y+1]);
		}

		out
	}

	pub fn dim(&self) -> Point {
		Point{x: self.grid.len(), y: self.grid[0].len()}
	}
}

struct InternalTile{
	seen: bool,
	possible_occupants : Vec<PossibleOccupant>
} 

impl InternalTile{
	fn empty() -> InternalTile {
		InternalTile {seen: false, possible_occupants: Vec::new()}
	}
}

enum PossibleOccupant{
	Pit,
	Wampus
}

pub struct BoardAnalyzer<'a>{
	board: &'a Board,
	internal_board: InternalBoard
}

impl BoardAnalyzer<'_>{
	pub fn new(board: &Board) -> BoardAnalyzer {
		let mut out = BoardAnalyzer {
			board, 
			internal_board: InternalBoard::empty(board.dim())
		};
		out.observe(Point{x: 0, y: 0});
		out
	}

	fn observe(&mut self, p: Point){
		let p = p.change_perspective(self.board.grid.len());
		let mut int_tile = &mut self.internal_board.grid[p.x][p.y];
		int_tile.seen = true;

		let neighbors = self.internal_board.neighbors(p);

		let pub_tile = &self.board.grid[p.x][p.y];
		for percept in &pub_tile.passive_percepts {
			for neighbor in neighbors{
				neighbor.possible_occupants.push
			}
		}
	}
}