use super::Player;
use crate::{
    board::{
        chesspiece::{get_movements, ChessPieceType},
        field::Field,
        layout::{Board, BoardCoordinates},
    },
    common::{check_checkmate, check_if_king_in_check, is_not_checked_after_move, move_piece},
    interface::{get_input, CommandInput, GeneralInput},
};

use rand::Rng;

pub struct GameState {
    pub current_player: Player,
    pub board: Board,
    pub checkmate: bool,
}

impl GameState {
    pub fn start() -> Self {
        GameState {
            current_player: Player::User,
            board: Board::default().fill_standard_pieces(),
            checkmate: false,
        }
    }

    pub fn do_move(self, ai_engine: &mut impl crate::ai_engine::AI) -> Self {
        match self.current_player {
            Player::User => do_user_move(self),
            Player::Opponent => do_ai_move(self, ai_engine),
        }
    }

    pub fn switch_player(self) -> Self {
        Self {
            current_player: self.current_player.switch(),
            ..self
        }
    }
}

fn do_user_move(mut game_state: GameState) -> GameState {
    let king_in_check = check_if_king_in_check(&game_state.board, &game_state.current_player);

    if king_in_check && check_checkmate(game_state.board.clone(), &game_state.current_player) {
        game_state.checkmate = true;
        return game_state;
    }

    let (selected_field, selected_destination_field) = choose_fields(&game_state);
    game_state.board = move_piece(
        game_state.board,
        &selected_field,
        &selected_destination_field,
    );
    game_state
}

fn do_ai_move(mut game_state: GameState, ai_engine: &mut impl crate::ai_engine::AI) -> GameState {
    let king_in_check = check_if_king_in_check(&game_state.board, &game_state.current_player);

    if king_in_check && check_checkmate(game_state.board.clone(), &game_state.current_player) {
        game_state.checkmate = true;
        return game_state;
    }

    game_state.board = ai_engine.make_move(game_state.board);
    game_state
}

fn choose_fields(game_state: &GameState) -> (Field, Field) {
    let mut selected_field: Option<Field> = None;
    let mut selected_destination_field: Option<Field> = None;
    let mut possible_fields: Vec<BoardCoordinates> = vec![];
    loop {
        match (selected_field, selected_destination_field) {
            (None, None) => {
                let selected_field_coordinates: BoardCoordinates =
                    get_input("Select a field with your figure");
                let select_field = game_state.board.at(&selected_field_coordinates);
                if select_field.check_player() != Some(game_state.current_player) {
                    println!("This field doesn't have your figure.");
                } else {
                    possible_fields = get_movements(&select_field, &game_state.board);
                    if possible_fields.is_empty() {
                        println!("This figure has no possible moves!");
                    } else {
                        selected_field = Some(select_field);
                    }
                }
            }
            (Some(select_field), None) => {
                let input: GeneralInput =
                    get_input("Select a field to which you want to move your figure");
                if let GeneralInput::Command(CommandInput::Back) = input {
                    selected_field = None;
                    continue;
                }
                if let GeneralInput::Coordinates(coordinates) = input {
                    if possible_fields.contains(&coordinates) {
                        let destination_field = game_state.board.at(&coordinates);
                        if is_not_checked_after_move(
                            game_state.board.clone(),
                            &select_field,
                            &destination_field,
                        ) {
                            selected_destination_field = Some(destination_field);
                        } else {
                            println!("Invalid move, your King will be in check!")
                        }
                    }
                }
            }
            (Some(select_field), Some(destination_field)) => {
                break (select_field, destination_field)
            }
            _ => {
                // This should never happen
                (selected_field, selected_destination_field) = (None, None);
            }
        }
    }
}

fn do_computer_move(mut game_state: GameState) -> GameState {
    let king_in_check = check_if_king_in_check(&game_state.board, &game_state.current_player);

    if king_in_check && check_checkmate(game_state.board.clone(), &game_state.current_player) {
        game_state.checkmate = true;
        return game_state;
    }

    let possible_fields = game_state
        .board
        .get_all_fields_by_player(&game_state.current_player);

    let (selected_field, destination_field) = loop {
        let selected_field =
            possible_fields[rand::thread_rng().gen_range(0..possible_fields.len())];
        let possible_destinations = get_movements(&selected_field, &game_state.board);
        if !possible_destinations.is_empty() {
            let destination =
                possible_destinations[rand::thread_rng().gen_range(0..possible_destinations.len())];
            let destination_field = game_state.board.at(&destination);
            if is_not_checked_after_move(
                game_state.board.clone(),
                &selected_field,
                &destination_field,
            ) {
                break (selected_field, destination_field);
            }
        }
    };
    game_state.board = move_piece(game_state.board, &selected_field, &destination_field);

    game_state
}
