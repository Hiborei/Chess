use crate::{
    board::chesspiece::{get_movements, ChessPieceType},
    board::{field::Field, layout::Board},
    engine::Player,
};

/// This is made for common functions which can be used everywhere
///

#[allow(dead_code)]
pub fn repeat_in_place<T, F>(mut f: F, mut argument: T, times: u32) -> T
where
    F: FnMut(T) -> T,
{
    for _ in 0..times {
        argument = f(argument);
    }
    argument
}

// TODO: This doesn't fit into "common", but I don't know where I want it yet, maybe board?
pub fn check_if_king_in_check(board: &Board, current_player: &Player) -> bool {
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

pub fn check_checkmate(board: Board, current_player: &Player) -> bool {
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

pub fn move_piece(mut board: Board, selected: &Field, destination: &Field) -> Board {
    let piece = selected.piece.unwrap();
    board = board.remove_piece(selected.coordinates);
    board = board.add_replace_piece(destination.coordinates, piece);
    board
}

pub fn is_not_checked_after_move(
    board: Board,
    selected_field: &Field,
    selected_destination_field: &Field,
) -> bool {
    let board = move_piece(board, selected_field, selected_destination_field);
    let player = selected_field.check_player().unwrap();
    !check_if_king_in_check(&board, &player)
}
