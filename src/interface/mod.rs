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
