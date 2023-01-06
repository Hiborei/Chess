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

/*
struct LockableStateForests(Arc<(Mutex<Vec<Forest>>, Condvar)>);

impl LockableStateForests {
    fn lock(&self) -> LockedStateForests<'_> {
        LockedStateForests(self.0 .0.lock().unwrap())
    }
}

struct LockedStateForests<'a>(MutexGuard<'a, Vec<Forest>>);

impl<'a> LockedStateForests<'a> {
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Forest> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Node {
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    data: Option<Board>,
    score: i32,
}

impl Node {
    pub(crate) fn new(id: usize, data: Option<Board>, score: i32) -> Self {
        Self {
            id,
            parent: None,
            children: vec![],
            data,
            score,
        }
    }

    fn new_child(id: usize, parent_id: usize, data: Option<Board>, score: i32) -> Self {
        Self {
            id,
            parent: Some(parent_id),
            children: vec![],
            data,
            score,
        }
    }

    fn compare(&self, data: &Board) -> bool {
        if let Some(board) = &self.data {
            return board == data;
        }
        false
    }

    fn get_data(&self) -> Option<Board> {
        self.data.clone()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Forest(Vec<Node>);

impl Forest {
    pub(crate) fn new(root_node: Node) -> Self {
        Self(vec![root_node])
    }

    fn find_child(&self, parent_id: usize, data: Board) -> Option<Node> {
        for node in self.0.iter() {
            if Some(parent_id) == node.parent {
                if node.compare(&data) {
                    return Some(node.clone());
                }
            }
        }
        return None;
    }

    fn new_child(&mut self, parent_id: usize, data: Option<Board>, score: i32) -> usize {
        let child = Node::new_child(self.0.len(), parent_id, data, score);
        let id = child.id;
        self.0[parent_id].children.push(id);
        self.0.push(child);
        return id;
    }
}
*/
fn calculate_first_layers(
    board: Board,
    scoring_agent: impl ScoringAgent,
    depth_level: u32,
) -> Board {
    let boards = super::generate_options_for_current_board(&board, true);
    if boards.is_empty() {
        println!("Boards in creating first level are empty?");
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
                println!("User has no moves, I guess that's good?");
                return (i32::MAX, board);
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
    /*for board in boards {
        let mut best_value = i32::MIN;
        let boards = super::generate_options_for_current_board(&board, false);
        if boards.is_empty() {
            println!("User has no moves, I guess that's good?");
            return board;
        }
        for board in boards {
            let value = calculate_deeper_layers(board, true, 3, scoring_agent, depth_level, alfa, beta);
            best_value = cmp::max(best_value, value);
            alfa = cmp::max(best_value, alfa);
            if beta <= alfa {
                break;
            }
        }
        if best_value > overall_best_value {
            best_board = board;
            overall_best_value = best_value;
        }
    }*/
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
            return best_value;
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
            return best_value;
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
/*
fn level(
    forest: &mut Forest,
    board: Board,
    parent_id: usize,
    current_player_is_ai: bool,
    current_depth: u32,
    scoring_agent: impl ScoringAgent,
    depth_level: u32,
    test_first_try: bool,
) {
    if test_first_try {
        let mut now = Instant::now();

        let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
        println!("Generate options took {} us", now.elapsed().as_micros());
        now = Instant::now();
        match current_depth {
            1 => {
                for board in boards {
                    now = Instant::now();
                    let score = scoring_agent.score(&board);

                    println!("Score took {} us", now.elapsed().as_micros());
                    now = Instant::now();
                    forest.new_child(parent_id, Some(board.clone()), score);
                    println!("Creating new child took {} us", now.elapsed().as_micros());
                    now = Instant::now();
                    level(
                        forest,
                        board,
                        parent_id,
                        !current_player_is_ai,
                        current_depth + 1,
                        scoring_agent,
                        depth_level,
                        false,
                    );

                    println!("Doing next level took {} us", now.elapsed().as_micros());
                }
            }
            depth if depth < depth_level => {
                for board in boards {
                    let score = scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                    level(
                        forest,
                        board,
                        parent_id,
                        !current_player_is_ai,
                        depth + 1,
                        scoring_agent,
                        depth_level,
                        false,
                    );
                }
            }
            _ => {
                for board in boards {
                    let score = scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                }
            }
        }
    } else {
        let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
        match current_depth {
            1 => {
                for board in boards {
                    forest.new_child(parent_id, Some(board.clone()), score);
                    level(
                        forest,
                        board,
                        parent_id,
                        !current_player_is_ai,
                        current_depth + 1,
                        scoring_agent,
                        depth_level,
                        false,
                    );
                }
            }
            depth if depth < depth_level => {
                for board in boards {
                    let score = scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                    level(
                        forest,
                        board,
                        parent_id,
                        !current_player_is_ai,
                        depth + 1,
                        scoring_agent,
                        depth_level,
                        false,
                    );
                }
            }
            _ => {
                for board in boards {
                    let score = scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                }
            }
        }
    }
}
*/
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

// General idea:
// - have an engine which works on separate thread
// - on every new game state (after user chooses a move), lock mutex, update board, notify thread to wake up
// - when thread wakes up, it locks the mutex and calculates X next possible states, when finished -> wait for wakeup
// Is this possible to do with these since Min max should theoretically cut off some branches? (Research whether the cut off ones are possible at all)
