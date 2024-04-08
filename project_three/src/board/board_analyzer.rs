#![allow(dead_code)]
use crate::board::{Board};

pub struct BoardAnalyzer<'a>{
	board: &'a Board,
	internal_board: Board
}

impl BoardAnalyzer<'_>{
	pub fn new(board: &Board) -> BoardAnalyzer {
		BoardAnalyzer {
			board, 
			internal_board: Board::empty(board.grid.len(), board.grid[0].len())
		}
	}

}