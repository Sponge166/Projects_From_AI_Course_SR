#![allow(dead_code)]
use crate::board::Occupant;
use crate::board::percepts::PassivePercept;
use crate::board::{Board};
use Vec;
use crate::board::point::Point;

#[derive(Debug)]
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

#[derive(Debug)]
struct InternalTile{
	seen: bool, // true if tile has been visited
	empty: Option<bool>, // this will be true if we either visited the tile and there were no occupants 
	// or we visited a nieghboring tile and there were no percepts present 
	m_pit: Option<bool>, // same as m_wampus except for pits and breezes
	m_wampus: Option<bool>, // used to respresent the tile state of maybe wampus? 
	// when a given tile was next to stink percept 
	// but we lack the necessary information to be certain it contains a wampus 
	definite_occupant: Option<PossibleOccupant>,
} 

impl InternalTile{
	fn empty() -> InternalTile {
		InternalTile {seen: false, empty: None, m_pit: None, m_wampus: None, definite_occupant: None,}
	}
}

#[derive(Debug, Copy)]
#[derive(Clone)]
enum PossibleOccupant{
	Pit,
	Wampus
}

#[derive(Debug)]
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
		out.observe(Point{x: 0, y: 0}.board_perspective(board.grid.len()));
		out
	}

	pub fn observe(&mut self, p: Point){
	
		let neighbors = self.internal_board.neighbors(p);

		let int_tile = &mut self.internal_board.grid[p.x][p.y];

		if int_tile.seen {
			return ();
		}

		let pub_tile = &self.board.grid[p.x][p.y];

		int_tile.seen = true;
		if let Some(Occupant::Wampus(_)) = self.board.grid[p.x][p.y].occupant{
			int_tile.empty = Some(false);
		}
		else{
			int_tile.empty = Some(true);
		}

		if pub_tile.passive_percepts.len() == 0 {
			// neighbors are empty
			for np in &neighbors{
				let neighbor = &mut self.internal_board.grid[np.x][np.y];
				neighbor.empty = Some(true);
				neighbor.m_wampus = Some(false);
				neighbor.m_pit = Some(false);
			}

			return ();
		}
		
		let mut bp = false;
		let mut sp = false;

		for percept in &pub_tile.passive_percepts {
			let func = match **percept {
				PassivePercept::Breeze(_) => {
					bp = true;
					|n: &mut InternalTile| {if let None = n.m_pit {n.m_pit = Some(true);}}
				},
				PassivePercept::Stink(_) => {
					sp = true;
					|n: &mut InternalTile| {if let None = n.m_wampus {n.m_wampus = Some(true);}}
				},
				_ => continue
			};

			for np in &neighbors{
				let neighbor = &mut self.internal_board.grid[np.x][np.y];
				
				if let None = neighbor.empty{
					func(neighbor);
				}
			}
		}

		for np in &neighbors{
			let neighbor = &mut self.internal_board.grid[np.x][np.y];
			let npp = np.player_perspective(self.board.grid.len());
			if let None = neighbor.empty{
				if !bp { 
					if let Some(true) = neighbor.m_pit{
						neighbor.empty = Some(true);
						println!("Room ({}, {}) is safe", npp.x+1, npp.y+1);
					}
					neighbor.m_pit = Some(false)
				}
				if !sp { 
					if let Some(true) = neighbor.m_wampus{
						neighbor.empty = Some(true);
						println!("Room ({}, {}) is safe", npp.x+1, npp.y+1);
					}
					neighbor.m_wampus = Some(false)
				}
			}
		}

	}

	pub fn advise(&mut self, p: Point) {
		let new_defs = self.analyze();

		let neighbors = self.internal_board.neighbors(p);

		for point in neighbors{
			let neighbor = &self.internal_board.grid[point.x][point.y];

			if let Some(true) = neighbor.empty {
				continue;
			}

			let pp = point.player_perspective(self.internal_board.grid.len());
			if let Some(x) = neighbor.definite_occupant {
				if new_defs.contains(&point) {continue;} // dont print twice
				match x {
					PossibleOccupant::Wampus => {
						println!("Room ({}, {}) contains the Wampus", pp.x+1, pp.y+1);
					},
					PossibleOccupant::Pit => {
						println!("Room ({}, {}) contains a Pit", pp.x+1, pp.y+1);
					}
				}
			}
			else{
				if let Some(true) = neighbor.m_wampus {
					println!("Room ({}, {}) MAY contain the Wampus", pp.x+1, pp.y+1);
				}
				if let Some(true) = neighbor.m_pit {
					println!("Room ({}, {}) MAY contain a Pit", pp.x+1, pp.y+1);
				}
			}
		}
	}

	fn analyze(&mut self) -> Vec<Point>{
		let mut out: Vec<Point> = Vec::new();

		for i in 0..self.internal_board.grid.len(){
			for j in 0..self.internal_board.grid[0].len(){
				let p = Point {x: i, y: j};

				let m_wampus: Option<bool>;
				let m_pit: Option<bool>;

				{ // skip conditions
					let t = &self.internal_board.grid[p.x][p.y];
					if let Some(true) = t.empty { continue; }
					if t.seen {continue;}
					if let (Some(false), Some(false)) = (t.m_wampus, t.m_pit) { continue; }
					if let Some(_) = t.definite_occupant { continue; }

					m_wampus = t.m_wampus;
					m_pit = t.m_pit;
				}

				if let Some(true) = m_wampus { 
					if self.is_def(p, PossibleOccupant::Wampus) {
						out.push(p);
						let pp = p.player_perspective(self.internal_board.grid.len());
						println!("Room ({}, {}) contains the Wampus", pp.x+1, pp.y+1);
						self.internal_board.grid[p.x][p.y].definite_occupant = Some(PossibleOccupant::Wampus);
					}
				}
				if let Some(true) = m_pit {
					if self.is_def(p, PossibleOccupant::Pit){
						out.push(p);
						let pp = p.player_perspective(self.internal_board.grid.len());
						println!("Room ({}, {}) contains a Pit", pp.x+1, pp.y+1);
						self.internal_board.grid[p.x][p.y].definite_occupant = Some(PossibleOccupant::Pit);
					}
				}
			}
		}

		out
	}

	fn is_def(&mut self, p: Point, po: PossibleOccupant) -> bool{

		// p definitely contains po if at least one of p's neighbors "can only come from" p

		let percept_neighbors = self.internal_board.neighbors(p);

		for pn in percept_neighbors {
			// println!("treating ({}, {}) as potential source of ({}, {})", p.x, p.y, pn.x, pn.y);
			if self.can_only_come_from(pn, p, po){
				return true;
			}
		}

		false
	}

	fn can_only_come_from(&mut self, percept_neighbor: Point, p: Point, po: PossibleOccupant) -> bool{
		{
			let pn_tile = &self.internal_board.grid[percept_neighbor.x][percept_neighbor.y];
			if !pn_tile.seen {
				// println!("ignored: UNSEEN");
				return false;
			} 
			// if this tile is unseen the player doesnt even know if it contains the percept we care about
			// and if the player doesnt know neither does board_analyzer.
		}

		let pn_neighbors = self.internal_board.neighbors(percept_neighbor);

		let func = match po {
			PossibleOccupant::Wampus => {
				|it: &InternalTile| if let Some(PossibleOccupant::Wampus) = it.definite_occupant {return true;} else{false}
			},
			PossibleOccupant::Pit => {
				|it: &InternalTile| if let Some(PossibleOccupant::Pit) = it.definite_occupant {return true;} else{false}
			},
		};

		for pnn in &pn_neighbors {
			if *pnn == p {continue;}
			let tile = &self.internal_board.grid[pnn.x][pnn.y];

			if func(tile) {return false;}

			let m_po = match po {
				PossibleOccupant::Wampus => tile.m_wampus,
				PossibleOccupant::Pit => tile.m_pit
			};

			// let st = match po {
			// 	PossibleOccupant::Wampus => "wampus",
			// 	PossibleOccupant::Pit => "pit"
			// };

			// println!("({}, {}) empty = {:?}, m_{st} = {:?}", pnn.x, pnn.y, tile.empty, m_po);

			if let (None, Some(true)) = (tile.empty, m_po) {
				return false;
			}
		}

		for pnn in &pn_neighbors {
			if *pnn == p {continue;}
			let tile = &mut self.internal_board.grid[pnn.x][pnn.y];
			match po {
				PossibleOccupant::Wampus => tile.m_wampus = Some(false),
				PossibleOccupant::Pit => tile.m_pit = Some(false)
			};

		}

		// println!("supposedly ({}, {}) passed can_only_come_from", percept_neighbor.x, percept_neighbor.y);

		true
	}
}