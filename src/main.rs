#![feature(mixed_integer_ops)]
#![feature(option_result_contains)]
#![feature(result_option_inspect)]
pub mod ai_engine;
mod board;
mod common;
mod engine;
mod interface;
use std::{
    io::{self, Write},
    mem,
};

use ai_engine::AI;
//use ai_engine::FlatBoard;
use engine::game_state::GameState;
use interface::board_layout::DrawInTerminal;

use crate::board::layout::Board;

fn main() {
    game_engine()
}

fn game_engine() {
    let mut game_state = GameState::start();
    let ai_engine = ai_engine::new_wrapper_for_min_max(
        ai_engine::DefaultAgent,
        game_state.board.clone(),
        false,
    );
    ai_engine::start(ai_engine.clone());
    loop {
        let _ = std::process::Command::new("clear").status();
        game_state.board.draw();
        println!("{}", mem::size_of::<Board>());
        io::stdout().flush().unwrap();
        game_state = game_state.do_move(ai_engine.clone());
        game_state = game_state.switch_player();
        if game_state.checkmate {
            println!("Checkmate! {} won!", game_state.current_player);
            break;
        }
    }
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

#[test]
fn board_placement_test() {
    use board::chesspiece::ChessPiece;
    use board::layout::{Board, BoardCoordinates};
    let mut board = Board::default();
    let pawn_move_coordinates = BoardCoordinates::from_coordinates(1, 1).unwrap();
    let bishop_move_coordinates = BoardCoordinates::from_coordinates(1, 2).unwrap();
    board = board
        .add_replace_piece(
            pawn_move_coordinates,
            ChessPiece {
                player: engine::Player::User,
                piece_type: board::chesspiece::ChessPieceType::Pawn,
            },
        )
        .add_replace_piece(
            bishop_move_coordinates,
            ChessPiece {
                player: engine::Player::User,
                piece_type: board::chesspiece::ChessPieceType::Bishop,
            },
        );

    board.draw();
}
