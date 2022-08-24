use std::fmt;
pub mod game_state;
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    User,
    Opponent,
}

impl Player {
    pub fn switch(self) -> Self {
        if self == Player::Opponent {
            Player::User
        } else {
            Player::Opponent
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::User => write!(f, "You"),
            Player::Opponent => write!(f, "Opponent"),
        }
    }
}
