use crate::engine::Player;

use super::{chesspiece::ChessPiece, layout::BoardCoordinates};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field {
    pub piece: Option<ChessPiece>,
    pub coordinates: BoardCoordinates,
}

impl Field {
    pub fn new(index: u32) -> Field {
        Field {
            piece: None,
            coordinates: BoardCoordinates::from_index(index).unwrap(),
        }
    }

    pub fn add_replace_piece(&mut self, piece: ChessPiece) {
        self.piece = Some(piece)
    }

    pub fn remove_piece(&mut self) {
        self.piece = None;
    }

    pub fn check_player(&self) -> Option<Player> {
        self.piece.map(|piece| piece.player)
    }
}
