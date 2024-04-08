use board::{Board};

struct Board_Analyzer{
	board: Board
}

impl Board_Analyzer{
	fn new(board: Board) -> Board_Analyzer {
		Board_Analyzer {board}
	}
}