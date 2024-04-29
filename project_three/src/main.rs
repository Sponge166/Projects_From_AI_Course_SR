use crate::player::Player;
use std::env;
use text_io::read;
use crate::board::board_analyzer::BoardAnalyzer;
pub mod board;
pub mod player;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2);

    let file_path = &args[1];

    let board = board::Board::from_file(file_path).expect("damn your file's messed up bruh");

    let l = board.grid.len();

    let mut board_analyzer = BoardAnalyzer::new(&board);

    let mut player = Player::new(&board);

    println!("{:?}", board);
    player.observe();

    loop{
        print!("Enter Move [L, R, F, S]: ");
        let mov: String = read!();

        match mov.to_lowercase().as_str() {
            "l" => player.turn_left(),
            "r" => player.turn_right(),
            "f" => player.forward(),
            "s" => player.shoot(),
            _ => continue
        };

        if player.board.wincon(player.pos.change_perspective(l)){
            println!("You see a glitter.. You're in the Gold room, you win!");
            break;
        }
        if player.board.wampus_losecon(player.pos.change_perspective(l)){
            println!("You see a set eyes and steamy breath.. You're in the Wampus' room, you lose!");
            break;
        }
        if player.board.pit_losecon(player.pos.change_perspective(l)){
            println!("You step foward into a new room and begin to fall into a pit, you lose!");
            break;
        }

        player.observe();
        board_analyzer.observe(player.pos.change_perspective(l));
        board_analyzer.advise(player.pos.change_perspective(l));

    }


}
