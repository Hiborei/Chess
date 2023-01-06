pub mod simple_min_max;

use crate::{
    board::{chesspiece::get_movements, layout::Board},
    common::{check_checkmate, check_if_king_in_check, move_piece},
    engine::Player,
};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use self::simple_min_max::MinMaxAI;

pub trait AI: Send + Sync {
    fn standby(&mut self);
    fn make_move(&mut self, board: Board) -> Board;
}

// pub struct AIWrapper<T: AI> {
//     engine: Arc<(Mutex<T>, Condvar)>,
// }

// impl<T: AI> AIWrapper<T> {}

pub fn start(engine_wrapper: Arc<(Mutex<MinMaxAI<DefaultAgent>>, Condvar)>) -> JoinHandle<()> {
    std::thread::spawn(move || loop {
        let mut engine = engine_wrapper.0.lock().unwrap();
        engine.standby();
        engine_wrapper.1.wait(engine).unwrap();
    })
}

pub fn make_move(engine_wrapper: Arc<(Mutex<impl AI>, Condvar)>, board: Board) -> Board {
    let mut engine = engine_wrapper.0.lock().unwrap();
    let new_board = engine.make_move(board);
    engine_wrapper.1.notify_one();
    new_board
}

pub fn new_wrapper_for_min_max(
    scoring_agent: DefaultAgent,
    board: Board,
    first_move: bool,
) -> Arc<(Mutex<simple_min_max::MinMaxAI<DefaultAgent>>, Condvar)> {
    let min_max = if first_move {
        simple_min_max::MinMaxAI {
            current_state: board.clone(),
            state_trees: vec![(board.clone(), board)],
            scoring_agent,
            depth_level: 5,
        }
    } else {
        simple_min_max::MinMaxAI {
            current_state: board,
            state_trees: vec![],
            scoring_agent,
            depth_level: 5,
        }
    };
    Arc::new((Mutex::new(min_max), Condvar::new()))
}

pub fn generate_options_for_current_board(board: &Board, player_is_ai: bool) -> Vec<Board> {
    let player = if player_is_ai {
        Player::Opponent
    } else {
        Player::User
    };
    let mut boards = vec![];
    let king_in_check = check_if_king_in_check(board, &player);

    if king_in_check && check_checkmate(board.clone(), &player) {
        // There's nothing more to do, it's checkmate
        return vec![];
    }

    let possible_fields = board.get_all_fields_by_player(&player);

    for selected_field in possible_fields {
        let possible_destinations = get_movements(&selected_field, board);
        for selected_destination in possible_destinations {
            let destination_field = board.at(&selected_destination);
            let new_board = move_piece(board.clone(), &selected_field, &destination_field);

            if !check_if_king_in_check(&new_board, &player) {
                boards.push(new_board);
            }
        }
    }
    boards
}

pub trait ScoringAgent: Send + Sync + Copy {
    fn score(&self, board: &Board) -> i32;
}

#[derive(Copy, Clone)]
pub struct DefaultAgent;

impl ScoringAgent for DefaultAgent {
    // TEMPORARY, JUST FOR TESTS
    fn score(&self, board: &Board) -> i32 {
        let mut ai_pieces = 0;
        let mut user_pieces = 0;
        for field in board.0 {
            if let Some(piece) = field.piece {
                if piece.player == Player::User {
                    user_pieces += piece.piece_type.value();
                } else {
                    ai_pieces += piece.piece_type.value();
                }
            }
        }
        ai_pieces - user_pieces
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
