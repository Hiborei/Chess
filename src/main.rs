#![feature(mixed_integer_ops)]
#![feature(option_result_contains)]
#![feature(result_option_inspect)]
mod board;
mod common;
mod engine;
mod interface;
use std::io::{self, Write};

use board::layout::Board;
use engine::game_state::GameState;
use interface::board_layout::DrawInTerminal;

use crate::board::chesspiece::ChessPiece;
fn main() {
    game_engine()
}

fn game_engine() {
    let mut game_state = GameState::start();
    loop {
        std::process::Command::new("clear").status();
        game_state.board.draw();

        io::stdout().flush().unwrap();
        game_state = game_state.do_move();
        game_state = game_state.switch_player();
        if game_state.checkmate {
            println!("Congratulations! {} has won!", game_state.current_player);
            break;
        }
    }
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

#[test]
fn board_placement_test() {
    use board::layout::BoardCoordinates;
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
