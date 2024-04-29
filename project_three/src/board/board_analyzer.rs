#![allow(dead_code)]
use crate::board::percepts::PassivePercept;
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

	fn neighbors(&mut self, p: Point) -> Vec<Point>{
		let mut out = Vec::new();
		let d = self.dim();

		if p.x > 0 {
			out.push(Point{ x: p.x-1, y: p.y});
		}
		if p.y > 0 {
			out.push(Point{ x: p.x, y: p.y-1});
		}
		if p.x < d.x -1 {
			out.push(Point{ x: p.x+1, y: p.y});		
		}
		if p.y < d.x -1 {
			out.push(Point{ x: p.x, y: p.y+1});
		}

		out
	}

	pub fn dim(&self) -> Point {
		Point{x: self.grid.len(), y: self.grid[0].len()}
	}
}

struct InternalTile{
	seen: bool, // true if tile has been visited
	empty: bool, // this will be true if we either visited the tile and there were no occupants 
	// or we visited a nieghboring tile and there were no percepts present 
	m_pit: bool, // same as m_wampus except for pits and breezes
	m_wampus: bool, // used to respresent the tile state of maybe wampus? 
	// when a given tile was next to stink percept 
	// but we lack the necessary information to be certain it contains a wampus 
	definite_occupant: Option<PossibleOccupant>,
} 

impl InternalTile{
	fn empty() -> InternalTile {
		InternalTile {seen: false, empty: false, m_pit: false, m_wampus: false, definite_occupant: None,}
	}
}

#[derive(Debug, Copy)]
#[derive(Clone)]
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
		out.observe(Point{x: 0, y: 0}.change_perspective(board.grid.len()));
		out
	}

	pub fn observe(&mut self, p: Point){
		let int_tile = &mut self.internal_board.grid[p.x][p.y];

		if int_tile.seen {
			return ();
		}

		int_tile.seen = true;
		int_tile.empty = true;

		let neighbors = self.internal_board.neighbors(p);

		let pub_tile = &self.board.grid[p.x][p.y];

		if pub_tile.passive_percepts.len() == 0 {
			// neighbors are empty
			for np in &neighbors{
				let neighbor = &mut self.internal_board.grid[np.x][np.y];
				neighbor.empty = true;
			}

			return ();
		}

		for percept in &pub_tile.passive_percepts {
			let func = match **percept {
				PassivePercept::Breeze(_) => |n: &mut InternalTile| n.m_pit = true,
				PassivePercept::Stink(_) => |n: &mut InternalTile| n.m_wampus = true,
				_ => continue
			};

			for np in &neighbors{
				let neighbor = &mut self.internal_board.grid[np.x][np.y];
				
				if !neighbor.empty{
					func(neighbor);
				}
			}
		}
	}

	pub fn advise(&mut self, p: Point) {
		let new_defs = self.analyze();

		let neighbors = self.internal_board.neighbors(p);
		let p_tile = &self.internal_board.grid[p.x][p.y];

		for point in neighbors{
			let neighbor = &self.internal_board.grid[point.x][point.y];

			if neighbor.empty {
				continue;
			}
			let pp = point.change_perspective(self.internal_board.grid.len());
			if let Some(x) = neighbor.definite_occupant {
				if new_defs.contains(&point) {continue;} // dont print twice
				match x {
					PossibleOccupant::Wampus => {
						println!("Room ({}, {}) contains the Wampus", pp.x, pp.y);
					},
					PossibleOccupant::Pit => {
						println!("Room ({}, {}) contains a Pit", pp.x, pp.y);
					}
				}
			}
			else {
				if p_tile.m_wampus {
					println!("Room ({}, {}) MAY contain the Wampus", pp.x, pp.y);
				}
				if p_tile.m_pit {
					println!("Room ({}, {}) MAY contain a Pit", pp.x, pp.y);
				}
			}


		}
	}

	fn analyze(&mut self) -> Vec<Point>{
		let mut out: Vec<Point> = Vec::new();

		for i in 0..self.internal_board.grid.len(){
			for j in 0..self.internal_board.grid[0].len(){
				let p = Point {x: i, y: j};

				let m_wampus: bool;
				let m_pit: bool;

				{ // skip conditions
					let t = &self.internal_board.grid[p.x][p.y];
					if t.empty || t.seen { continue; }
					if !t.m_wampus && !t.m_pit { continue; }
					if let Some(_) = t.definite_occupant { continue; }

					m_wampus = t.m_wampus;
					m_pit = t.m_pit;
				}

				if m_wampus && self.is_def(p, PossibleOccupant::Wampus) {
					out.push(p);
					let pp = p.change_perspective(self.internal_board.grid.len());
					println!("Room ({}, {}) contains the Wampus", pp.x, pp.y);
					self.internal_board.grid[p.x][p.y].definite_occupant = Some(PossibleOccupant::Wampus);
				}
				if m_pit && self.is_def(p, PossibleOccupant::Pit){
					out.push(p);
					let pp = p.change_perspective(self.internal_board.grid.len());
					println!("Room ({}, {}) contains a Pit", pp.x, pp.y);
					self.internal_board.grid[p.x][p.y].definite_occupant = Some(PossibleOccupant::Pit);
				}
			}
		}

		out
	}

	fn is_def(&mut self, p: Point, po: PossibleOccupant) -> bool{

		// p definitely contains po if at least one of p's neighbors "can only come from" p

		let percept_neighbors = self.internal_board.neighbors(p);

		for pn in percept_neighbors {
			if self.can_only_come_from(pn, p, po){
				return true;
			}
		}

		false
	}

	fn can_only_come_from(&mut self, percept_neighbor: Point, p: Point, po: PossibleOccupant) -> bool{
		let pn_neighbors = self.internal_board.neighbors(percept_neighbor);

		let func = match po {
			PossibleOccupant::Wampus => {
				|it: &InternalTile| if let Some(PossibleOccupant::Wampus) = it.definite_occupant {return true;} else{false}
			},
			PossibleOccupant::Pit => {
				|it: &InternalTile| if let Some(PossibleOccupant::Pit) = it.definite_occupant {return true;} else{false}
			},
		};


		for pnn in pn_neighbors {
			if pnn == p {continue;}
			let tile = &self.internal_board.grid[pnn.x][pnn.y];

			if func(tile) {return false;}

			let m_po = match po {
				PossibleOccupant::Wampus => tile.m_wampus,
				PossibleOccupant::Pit => tile.m_pit
			};

			println!("({}, {}) empty = {}", pnn.x, pnn.y, tile.empty);

			if !tile.empty && m_po {
				return false;
			}
		}

		println!("supposedly ({}, {}) passed can_only_come_from", percept_neighbor.x, percept_neighbor.y);

		true
	}
}