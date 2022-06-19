use crate::engine::Player;

use super::{chesspiece::ChessPiece, layout::BoardCoordinates};

#[derive(Debug, Clone, Copy)]
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

    pub fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    pub fn remove_piece(&mut self) {
        self.piece = None;
    }

    pub fn check_player(&self) -> Option<Player> {
        if let Some(piece) = self.piece {
            Some(piece.player)
        } else {
            None
        }
    }
}
