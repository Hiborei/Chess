use std::io::{self, Write};

use crate::{
    board::{
        field::Field,
        layout::{Board, BoardCoordinates},
    },
    engine::Player,
};

extern crate termion;

use termion::{color, style};

pub trait DrawInTerminal {
    fn draw(&self);
}

impl DrawInTerminal for Board {
    fn draw(&self) {
        println!("");
        print!("  {}A|B|C|D|E|F|G|H|", color::Fg(color::White));
        for x in 0..8 {
            println!("");

            print!("{}|", x + 1);

            for y in 0..8 {
                self.at(&BoardCoordinates::from_coordinates(x, y).unwrap())
                    .draw();
                print!("{}|", color::Fg(color::White));
                io::stdout().flush().unwrap();
            }
        }

        println!("");
    }
}

impl DrawInTerminal for Field {
    fn draw(&self) {
        if let Some(piece) = self.piece {
            if matches!(piece.player, Player::User) {
                print!("{}{}", color::Fg(color::Blue), piece.piece_type)
            } else {
                print!("{}{}", color::Fg(color::Red), piece.piece_type)
            };
        } else {
            print!(" ");
        }
    }
}
