use super::Player;
use crate::{
    board::{
        chesspiece::{get_movements, ChessPieceType},
        field::Field,
        layout::{Board, BoardCoordinates},
    },
    interface::get_input,
};

use rand::Rng;
use termion::color;

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

    pub fn do_move(self) -> Self {
        match self.current_player {
            Player::User => do_user_move(self),
            Player::Opponent => do_computer_move(self),
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

    let selected_field_coordinates: BoardCoordinates = get_input("Select a field with your figure");
    let mut selected_field = game_state.board.at(&selected_field_coordinates);
    while selected_field.check_player() != Some(game_state.current_player) {
        println!("This field doesn't have your figure.");
        let selected_field_coordinates: BoardCoordinates =
            get_input("Select a field with your figure");
        selected_field = game_state.board.at(&selected_field_coordinates);
    }

    let possible_fields = get_movements(&selected_field, &game_state.board);
    println!(
        "{}[debug] {:?} {}",
        color::Fg(color::Green),
        possible_fields,
        color::Fg(color::White)
    );

    let selected_destination_field = loop {
        let selected_field_coordinates: BoardCoordinates =
            get_input("Select a field to which you want to move your figure");
        if possible_fields.contains(&selected_field_coordinates) {
            let selected_destination_field = game_state.board.at(&selected_field_coordinates);
            if is_not_checked_after_move(
                game_state.board.clone(),
                &selected_field,
                &selected_destination_field,
            ) {
                break selected_destination_field;
            }
        }
    };

    game_state.board = move_piece(
        game_state.board,
        &selected_field,
        &selected_destination_field,
    );
    game_state
}

fn is_not_checked_after_move(
    board: Board,
    selected_field: &Field,
    selected_destination_field: &Field,
) -> bool {
    let board = move_piece(board, selected_field, selected_destination_field);
    let player = selected_field.check_player().unwrap();
    !check_if_king_in_check(&board, &player)
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

fn check_if_king_in_check(board: &Board, current_player: &Player) -> bool {
    let fields = board.get_all_fields_by_player(&current_player.switch());

    for field in fields {
        if get_movements(&field, board)
            .into_iter()
            .filter_map(|coordinates| board.at(&coordinates).piece)
            .any(|chesspiece| chesspiece.piece_type == ChessPieceType::King)
        {
            return true;
        }
    }
    false
}

fn move_piece(mut board: Board, selected: &Field, destination: &Field) -> Board {
    let piece = selected.piece.unwrap();
    board = board.remove_piece(selected.coordinates);
    board = board.add_replace_piece(destination.coordinates, piece);
    board
}

fn check_checkmate(board: Board, current_player: &Player) -> bool {
    let fields = board.get_all_fields_by_player(current_player);
    for field in fields {
        for destination_coordinates in get_movements(&field, &board) {
            let temp_board = move_piece(board.clone(), &field, &board.at(&destination_coordinates));
            if !check_if_king_in_check(&temp_board, current_player) {
                return false;
            }
        }
    }
    true
}
