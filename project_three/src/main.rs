use crate::player::Player;
use std::env;
use crate::board::board_analyzer::BoardAnalyzer;
pub mod board;
pub mod player;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2);

    let file_path = &args[1];

    let board = board::Board::from_file(file_path).expect("damn your file's messed up bruh");

    let board_analyzer = BoardAnalyzer::new(&board);

    let player = Player::new(&board);

    println!("{:?}", board);
}
