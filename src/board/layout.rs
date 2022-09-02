use std::str::FromStr;

use crate::{engine::Player, interface};

use super::{
    chesspiece::{ChessPiece, ChessPieceType},
    field::Field,
};

#[derive(Debug, Clone)]
pub struct Board(pub(crate) [Field; 64]);

impl Board {
    pub fn fill_standard_pieces(mut self) -> Self {
        for y in 0..8 {
            self = self
                .add_replace_piece(
                    BoardCoordinates::from_coordinates(6, y).unwrap(),
                    ChessPiece {
                        piece_type: super::chesspiece::ChessPieceType::Pawn,
                        player: Player::User,
                    },
                )
                .add_replace_piece(
                    BoardCoordinates::from_coordinates(1, y).unwrap(),
                    ChessPiece {
                        piece_type: super::chesspiece::ChessPieceType::Pawn,
                        player: Player::Opponent,
                    },
                )
        }
        self = self
            .add_replace_piece(
                BoardCoordinates::from_index(56).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Rook,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(57).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Knight,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(58).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Bishop,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(59).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Queen,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(60).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::King,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(61).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Bishop,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(62).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Knight,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(63).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Rook,
                    player: Player::User,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(0).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Rook,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(1).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Knight,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(2).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Bishop,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(3).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Queen,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(4).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::King,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(5).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Bishop,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(6).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Knight,
                    player: Player::Opponent,
                },
            )
            .add_replace_piece(
                BoardCoordinates::from_index(7).unwrap(),
                ChessPiece {
                    piece_type: super::chesspiece::ChessPieceType::Rook,
                    player: Player::Opponent,
                },
            );
        self
    }

    pub(crate) fn at_mut(&mut self, coordinates: &BoardCoordinates) -> &mut Field {
        &mut self.0[(coordinates.x * 8 + coordinates.y) as usize]
    }

    pub(crate) fn at(&self, coordinates: &BoardCoordinates) -> Field {
        self.0[(coordinates.x * 8 + coordinates.y) as usize]
    }

    pub fn get_all_fields_by_player(&self, player: &Player) -> Vec<Field> {
        self.0
            .into_iter()
            .filter(|field| field.check_player().contains(player))
            .collect()
    }

    pub(crate) fn remove_piece(mut self, coordinates: BoardCoordinates) -> Board {
        self.at_mut(&coordinates).remove_piece();
        self
    }

    pub(crate) fn add_replace_piece(
        mut self,
        coordinates: BoardCoordinates,
        mut piece: super::chesspiece::ChessPiece,
    ) -> Board {
        if piece.can_change(coordinates) {
            let piece_type: ChessPieceType =
                interface::get_input("Choose a piece to replace the pawn: ");
            piece.piece_type = piece_type
        }
        self.at_mut(&coordinates).add_replace_piece(piece);
        self
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BoardCoordinates {
    x: u32,
    y: u32,
}

impl BoardCoordinates {
    pub fn from_index(index: u32) -> Option<Self> {
        if index > 63 {
            None
        } else {
            Some(Self {
                x: index / 8,
                y: index % 8,
            })
        }
    }

    pub fn from_coordinates(x: u32, y: u32) -> Option<Self> {
        if x > 7 || y > 7 {
            None
        } else {
            Some(Self { x, y })
        }
    }

    pub fn up(self) -> Option<Self> {
        if self.x > 0 {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    pub fn down(self) -> Option<Self> {
        if self.x < 7 {
            Some(Self {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    pub fn left(self) -> Option<Self> {
        if self.y > 0 {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    pub fn right(self) -> Option<Self> {
        if self.y < 7 {
            Some(Self {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }
}

impl FromStr for BoardCoordinates {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Inputted String for Board Coordinates is too short!".to_owned());
        }
        let letter = s.get(..1).unwrap();
        let y = match letter {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            "D" => 3,
            "E" => 4,
            "F" => 5,
            "G" => 6,
            "H" => 7,
            _ => return Err(format!("Invalid letter for Board Coordinates: {}", letter)),
        };
        let digit = s.get(1..2).unwrap();
        let x = digit
            .parse::<u32>()
            .map_err(|_| format!("Invalid digit for Board Coordinates: {}", digit))
            .and_then(|digit| {
                digit
                    .checked_sub(1)
                    .ok_or_else(|| format!("Invalid digit for Board Coordinates: {}", digit))
            })?;
        let coordinates =
            Self::from_coordinates(x, y).ok_or_else(|| "Invalid Board Coordinates!".to_owned());
        print!("[debug] Coordinates: {:?}", coordinates);
        coordinates
    }
}

#[test]

fn coordinates_check() {
    let coordinates = BoardCoordinates::from_index(5).unwrap();
    assert_eq!(BoardCoordinates { x: 0, y: 5 }, coordinates);
    let coordinates = BoardCoordinates::from_index(12).unwrap();
    assert_eq!(BoardCoordinates { x: 1, y: 4 }, coordinates);
}

impl Default for Board {
    fn default() -> Self {
        Board(array_init::array_init::<_, Field, 64>(|i| {
            Field::new(i as u32)
        }))
    }
}
