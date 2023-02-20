//! Minmax algorithm with alpha-beta pruning.
//! When idle, it will generate all movement possibilities recurrently.
//! After reaching the selected depth it will return the final board value.

use super::{ScoringAgent, AI};
use crate::board::layout::Board;
use rayon::prelude::*;
use std::cmp;

pub struct MinMaxAI<T: ScoringAgent + 'static> {
    pub(crate) state_trees: Vec<(Board, Board)>,
    pub(crate) scoring_agent: T,
    pub(crate) current_state: Board,
    pub(crate) depth_level: u32,
}

fn calculate_first_layers(
    board: Board,
    scoring_agent: impl ScoringAgent,
    depth_level: u32,
) -> Board {
    let boards = super::generate_options_for_current_board(&board, true);
    if boards.is_empty() {
        //println!("Boards in creating first level are empty?");
        return board;
    }
    let final_result = boards
        .into_par_iter()
        .map(|board| {
            let mut alfa = i32::MIN;
            let beta = i32::MAX;
            let mut best_value = i32::MIN;
            let boards = super::generate_options_for_current_board(&board, false);
            if boards.is_empty() {
                //println!("User has no moves, I guess that's good?");
                return (i32::MAX - 1, board);
            }
            for board in boards {
                let value =
                    calculate_deeper_layers(board, true, 3, scoring_agent, depth_level, alfa, beta);
                best_value = cmp::max(best_value, value);
                alfa = cmp::max(best_value, alfa);
                if beta <= alfa {
                    break;
                }
            }
            (best_value, board)
        })
        .reduce(
            || (i32::MIN, Board::default()),
            |(best_value, best_board), (value, board)| {
                if value > best_value {
                    (value, board)
                } else {
                    (best_value, best_board)
                }
            },
        );
    final_result.1
}

fn calculate_deeper_layers(
    board: Board,
    current_player_is_ai: bool,
    current_depth: u32,
    scoring_agent: impl ScoringAgent,
    depth_level: u32,
    mut alfa: i32,
    mut beta: i32,
) -> i32 {
    if current_depth == depth_level {
        return scoring_agent.score(&board);
    }

    if current_player_is_ai {
        let mut best_value = i32::MIN;
        let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
        if boards.is_empty() {
            return best_value + 1;
        }
        for board in boards {
            let value = calculate_deeper_layers(
                board,
                !current_player_is_ai,
                current_depth + 1,
                scoring_agent,
                depth_level,
                alfa,
                beta,
            );
            best_value = cmp::max(best_value, value);
            alfa = cmp::max(best_value, alfa);
            if beta <= alfa {
                break;
            }
        }
        best_value
    } else {
        let mut best_value = i32::MAX;
        let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
        if boards.is_empty() {
            return best_value - 1;
        }
        for board in boards {
            let value = calculate_deeper_layers(
                board,
                !current_player_is_ai,
                current_depth + 1,
                scoring_agent,
                depth_level,
                alfa,
                beta,
            );
            best_value = cmp::min(best_value, value);
            beta = cmp::min(best_value, beta);
            if beta <= alfa {
                break;
            }
        }
        best_value
    }
}

impl<T: ScoringAgent> MinMaxAI<T> {
    fn grow_forests(&mut self) {
        let current_player_is_ai = false;
        if self.state_trees.is_empty() {
            let boards = super::generate_options_for_current_board(
                &self.current_state,
                current_player_is_ai,
            );
            for board in boards {
                let result_board =
                    calculate_first_layers(board.clone(), self.scoring_agent, self.depth_level);
                self.state_trees.push((board, result_board));
            }
        } else {
            // This means that we have the first move
            let result_board = calculate_first_layers(
                self.current_state.clone(),
                self.scoring_agent,
                self.depth_level,
            );
            self.state_trees.clear();
            self.state_trees
                .push((self.current_state.clone(), result_board))
        }
    }
}

impl<T: ScoringAgent> AI for MinMaxAI<T>
where
    T: Sync,
{
    fn make_move(&mut self, board: crate::board::layout::Board) -> crate::board::layout::Board {
        let mut it = self.state_trees.iter_mut();
        let correct_path = loop {
            let boards = it.next().expect("Somehow there's no path like this??");
            if boards.0 == board {
                break boards.1.clone();
            }
        };
        self.state_trees.clear();
        self.current_state = correct_path.clone();
        correct_path
    }

    fn standby(&mut self) {
        self.grow_forests();
    }
}
