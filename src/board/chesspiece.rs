use std::{fmt, str::FromStr};

use super::{
    field::Field,
    layout::{Board, BoardCoordinates},
};
use crate::engine::Player;

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: ChessPieceType,
    pub player: Player,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChessPieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub fn get_movements(field: &Field, board: &Board) -> Vec<BoardCoordinates> {
    if let Some(piece) = &field.piece {
        match piece.piece_type {
            ChessPieceType::Pawn => pawn_moves(board, field),
            ChessPieceType::Bishop => bishop_moves(board, field),
            ChessPieceType::Knight => knight_moves(board, field),
            ChessPieceType::Rook => rook_moves(board, field),
            ChessPieceType::Queen => queen_moves(board, field),
            ChessPieceType::King => king_moves(board, field),
        }
    } else {
        vec![]
    }
}

// THIS LOOKS LIKE TERRIBLE DESIGN HELP
pub fn front(coordinates: BoardCoordinates, player: Player) -> Option<BoardCoordinates> {
    match player {
        Player::User => coordinates.up(),
        Player::Opponent => coordinates.down(),
    }
}

impl fmt::Display for ChessPieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChessPieceType::Pawn => write!(f, "P"),
            ChessPieceType::Bishop => write!(f, "B"),
            ChessPieceType::Knight => write!(f, "N"),
            ChessPieceType::Rook => write!(f, "R"),
            ChessPieceType::Queen => write!(f, "Q"),
            ChessPieceType::King => write!(f, "K"),
        }
    }
}

impl FromStr for ChessPieceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.get(..1) {
            Some("P") => Ok(ChessPieceType::Pawn),
            Some("B") => Ok(ChessPieceType::Bishop),
            Some("N") => Ok(ChessPieceType::Knight),
            Some("R") => Ok(ChessPieceType::Rook),
            Some("Q") => Ok(ChessPieceType::Queen),
            Some("K") => Ok(ChessPieceType::King),
            _ => Err("Invalid chess piece".to_string()),
        }
    }
}

fn pawn_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = vec![];
    let current_player = current_field.check_player().unwrap();
    let own_coordinates = current_field.coordinates;

    if let Some(coordinates) = front(own_coordinates, current_player) {
        let field = board.at(&coordinates);
        if field.check_player().is_none() {
            coordinates_vec.push(coordinates);
        }
    }

    if let Some(coordinates) =
        front(own_coordinates, current_player).and_then(BoardCoordinates::right)
    {
        let field = board.at(&coordinates);
        if field.check_player().contains(&current_player.switch()) {
            coordinates_vec.push(coordinates);
        }
    }

    if let Some(coordinates) =
        front(own_coordinates, current_player).and_then(BoardCoordinates::left)
    {
        let field = board.at(&coordinates);
        if field.check_player().contains(&current_player.switch()) {
            coordinates_vec.push(coordinates);
        }
    }
    coordinates_vec
}

fn bishop_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = vec![];
    let current_player = current_field.check_player().unwrap();
    let own_coordinates = current_field.coordinates;
    let mut temp_coordinates = own_coordinates;

    while let Some(coordinates) = temp_coordinates.up().and_then(BoardCoordinates::left) {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }

        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.up().and_then(BoardCoordinates::right) {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.down().and_then(BoardCoordinates::left) {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.down().and_then(BoardCoordinates::right) {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    coordinates_vec
}

fn knight_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = vec![];
    let current_player = current_field.check_player().unwrap();
    let own_coordinates = current_field.coordinates;

    if let Some(coordinates) = own_coordinates.up().and_then(BoardCoordinates::up) {
        if let Some(coordinates) = coordinates.right() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.left() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }

    if let Some(coordinates) = own_coordinates.right().and_then(BoardCoordinates::right) {
        if let Some(coordinates) = coordinates.up() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.down() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }

    if let Some(coordinates) = own_coordinates.left().and_then(BoardCoordinates::left) {
        if let Some(coordinates) = coordinates.up() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.down() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }

    if let Some(coordinates) = own_coordinates.down().and_then(BoardCoordinates::down) {
        if let Some(coordinates) = coordinates.left() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.right() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }
    coordinates_vec
}

fn rook_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = vec![];
    let current_player = current_field.check_player().unwrap();
    let own_coordinates = current_field.coordinates;

    let mut temp_coordinates = own_coordinates;

    while let Some(coordinates) = temp_coordinates.up() {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }

        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.down() {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.left() {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    temp_coordinates = own_coordinates;
    while let Some(coordinates) = temp_coordinates.right() {
        let field = board.at(&coordinates);
        match field.check_player() {
            Some(player) if player == current_player => break,
            Some(_) => {
                coordinates_vec.push(coordinates);
                break;
            }
            None => coordinates_vec.push(coordinates),
        }
        temp_coordinates = coordinates;
    }

    coordinates_vec
}

fn queen_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = rook_moves(board, current_field);
    coordinates_vec.extend(bishop_moves(board, current_field));
    coordinates_vec
}

fn king_moves(board: &Board, current_field: &Field) -> Vec<BoardCoordinates> {
    let mut coordinates_vec = vec![];
    let current_player = current_field.check_player().unwrap();
    let own_coordinates = current_field.coordinates;

    if let Some(coordinates) = own_coordinates.up() {
        let field = board.at(&coordinates);
        if !field.check_player().contains(&current_player) {
            coordinates_vec.push(coordinates);
        }
        if let Some(coordinates) = coordinates.left() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.right() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }

    if let Some(coordinates) = own_coordinates.down() {
        let field = board.at(&coordinates);
        if !field.check_player().contains(&current_player) {
            coordinates_vec.push(coordinates);
        }
        if let Some(coordinates) = coordinates.left() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
        if let Some(coordinates) = coordinates.right() {
            let field = board.at(&coordinates);
            if !field.check_player().contains(&current_player) {
                coordinates_vec.push(coordinates);
            }
        }
    }

    if let Some(coordinates) = own_coordinates.left() {
        let field = board.at(&coordinates);
        if !field.check_player().contains(&current_player) {
            coordinates_vec.push(coordinates);
        }
    }

    if let Some(coordinates) = own_coordinates.right() {
        let field = board.at(&coordinates);
        if !field.check_player().contains(&current_player) {
            coordinates_vec.push(coordinates);
        }
    }

    coordinates_vec
}
impl ChessPiece {
    pub(crate) fn can_change(&self, coordinates: BoardCoordinates) -> bool {
        if self.piece_type == ChessPieceType::Pawn {
            match self.player {
                Player::User => coordinates.up().is_none(),
                Player::Opponent => coordinates.down().is_none(),
            }
        } else {
            false
        }
    }
}
