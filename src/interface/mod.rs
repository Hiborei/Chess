use std::str::FromStr;

use crate::board::{chesspiece::ChessPieceType, layout::BoardCoordinates};

pub mod board_layout;

pub fn get_input<T>(message: &str) -> T
where
    T: FromStr,
{
    loop {
        println!("{}", message);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if let Ok(input) = line.parse() {
            break input;
        }
        println!("Invalid, try again. {message}");
    }
}

impl FromStr for GeneralInput {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(command) = CommandInput::from_str(s) {
            Ok(Self::Command(command))
        } else if let Ok(coordinates) = BoardCoordinates::from_str(s) {
            Ok(Self::Coordinates(coordinates))
        } else if let Ok(chess_piece) = ChessPieceType::from_str(s) {
            Ok(Self::ChessPieceType(chess_piece))
        } else {
            Err(format!("Unknown input: {}", s))
        }
    }
}

#[derive(Debug)]
pub enum GeneralInput {
    Command(CommandInput),
    Coordinates(BoardCoordinates),
    ChessPieceType(ChessPieceType),
}

#[derive(Debug)]
pub enum CommandInput {
    Exit,
    Score, // Not used yet
    Back,
}

impl FromStr for CommandInput {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.to_ascii_lowercase();
        if command.starts_with("exit") {
            Ok(Self::Exit)
        } else if command.starts_with("score") {
            Ok(Self::Score)
        } else if command.starts_with("back") {
            Ok(Self::Back)
        } else {
            Err("Could not convert to CommandInput".to_owned())
        }
    }
}

impl From<CommandInput> for GeneralInput {
    fn from(command: CommandInput) -> Self {
        GeneralInput::Command(command)
    }
}

#[test]
fn test_from_strings() {
    let coordinates: BoardCoordinates = FromStr::from_str("A1").unwrap();
    assert_eq!(
        coordinates,
        BoardCoordinates::from_coordinates(0, 0).unwrap()
    );
    let piece_type: ChessPieceType = FromStr::from_str("K").unwrap();
    assert_eq!(piece_type, ChessPieceType::King);
}

#[test]
#[ignore = "Interactive"]
fn interactive_test() {
    let command: GeneralInput = get_input("Try a command");
    println!("{:?}", command);
}
