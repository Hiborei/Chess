use std::str::FromStr;

//use crate::board::layout::BoardCoordinates;

pub mod board_layout;

pub fn get_input<T>(message: &str) -> T
where
    T: FromStr,
{
    let input = loop {
        println!("{}", message);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if let Ok(input) = line.parse() {
            break input;
        } else {
            println!("Invalid, try again. {message}");
        }
    };
    input
}

/*
enum GeneralInput {
    Command(CommandInput),
    Coordinates(BoardCoordinates),
}

enum CommandInput {
    Exit,
    Score,
}
*/

#[test]
fn test_from_strings() {
    use crate::board::chesspiece::ChessPieceType;
    use crate::board::layout::BoardCoordinates;
    let coordinates: BoardCoordinates = FromStr::from_str("A1").unwrap();
    assert_eq!(
        coordinates,
        BoardCoordinates::from_coordinates(0, 0).unwrap()
    );
    let piece_type: ChessPieceType = FromStr::from_str("K").unwrap();
    assert_eq!(piece_type, ChessPieceType::King);
}
