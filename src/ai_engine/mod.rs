pub mod simple_min_max;

use crate::{
    board::{
        chesspiece::{get_movements, ChessPiece, ChessPieceType},
        field::Field,
        layout::Board,
    },
    common::{check_checkmate, check_if_king_in_check, is_not_checked_after_move, move_piece},
    engine::Player,
};

use rand::Rng;
use std::sync::mpsc;
use std::thread::JoinHandle;

pub trait AI {
    //fn start(&self) -> JoinHandle<()>;
    fn make_move(&mut self, board: Board) -> Board;
}

pub fn generate_options_for_current_board(board: &Board, player_is_ai: bool) -> Vec<Board> {
    let player = if player_is_ai {
        Player::Opponent
    } else {
        Player::User
    };
    let mut boards = vec![];
    let king_in_check = check_if_king_in_check(&board, &player);

    if king_in_check && check_checkmate(board.clone(), &player) {
        // There's nothing more to do, it's checkmate
        return vec![];
    }

    let possible_fields = board.get_all_fields_by_player(&player);

    for selected_field in possible_fields {
        let possible_destinations = get_movements(&selected_field, &board);
        for selected_destination in possible_destinations {
            let destination_field = board.at(&selected_destination);
            let new_board = move_piece(board.clone(), &selected_field, &destination_field);

            if !check_if_king_in_check(&new_board, &player) {
                boards.push(new_board);
            }
        }
    }
    return boards;
}

pub trait ScoringAgent: Send {
    fn score(&self, board: &Board) -> u32;
}

pub struct DefaultAgent;

impl ScoringAgent for DefaultAgent {
    // TEMPORARY, JUST FOR TESTS
    fn score(&self, board: &Board) -> u32 {
        let mut ai_pieces = 0;
        let mut user_pieces = 0;
        for field in board.0 {
            if let Some(piece) = field.piece {
                if piece.player == Player::User {
                    user_pieces += 1;
                } else {
                    ai_pieces += 1;
                }
            }
        }
        return ai_pieces - user_pieces;
    }
}

/*
#[derive(Debug, PartialEq, Clone)]
pub struct FlatBoard([u8; 64]);

impl From<Board> for FlatBoard {
    fn from(board: Board) -> Self {
        FlatBoard(board.0.map(|field| u8::from(field)))
    }
}

impl From<FlatBoard> for Board {
    fn from(board: FlatBoard) -> Self {
        let mut new_board = Board::default();
        for x in 0..64 {
            if let Some(piece) = ChessPiece::try_from(board.0[x]).ok() {
                new_board.0[x].add_replace_piece(piece);
            }
        }
        return new_board;
    }
}

impl From<Field> for u8 {
    fn from(field: Field) -> Self {
        if let Some(piece) = field.piece {
            u8::from(piece.piece_type) + u8::from(piece.player)
        } else {
            0
        }
    }
}

impl TryFrom<u8> for ChessPiece {
    type Error = ();
    fn try_from(field: u8) -> Result<Self, Self::Error> {
        if field > 8 {
            let piece = match field - 8 {
                1 => ChessPieceType::Pawn,
                2 => ChessPieceType::Knight,
                3 => ChessPieceType::Bishop,
                4 => ChessPieceType::Rook,
                5 => ChessPieceType::Queen,
                6 => ChessPieceType::King,
                _ => return Err(()),
            };
            Ok(ChessPiece {
                piece_type: piece,
                player: Player::User,
            })
        } else {
            let piece = match field {
                1 => ChessPieceType::Pawn,
                2 => ChessPieceType::Knight,
                3 => ChessPieceType::Bishop,
                4 => ChessPieceType::Rook,
                5 => ChessPieceType::Queen,
                6 => ChessPieceType::King,
                _ => return Err(()),
            };

            Ok(ChessPiece {
                piece_type: piece,
                player: Player::Opponent,
            })
        }
    }
}

impl From<ChessPieceType> for u8 {
    fn from(piece_type: ChessPieceType) -> Self {
        match piece_type {
            ChessPieceType::Pawn => 1,
            ChessPieceType::Knight => 2,
            ChessPieceType::Bishop => 3,
            ChessPieceType::Rook => 4,
            ChessPieceType::Queen => 5,
            ChessPieceType::King => 6,
        }
    }
}

impl From<Player> for u8 {
    fn from(player: Player) -> Self {
        match player {
            Player::User => 8,
            Player::Opponent => 0,
        }
    }
}

*/
