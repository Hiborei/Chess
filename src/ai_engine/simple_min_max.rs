use super::{ScoringAgent, AI};
use crate::board;
use crate::board::layout::Board;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::JoinHandle;

pub struct MinMaxAI<T: ScoringAgent + 'static> {
    pub(crate) state_trees: Vec<Forest>,
    pub(crate) scoring_agent: T,
    pub(crate) current_state: Board,
    pub(crate) depth_level: u32,
}

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

fn level(
    forest: &mut Forest,
    board: Board,
    parent_id: usize,
    current_player_is_ai: bool,
    current_depth: u32,
    scoring_agent: impl ScoringAgent,
    depth_level: u32,
) {
    let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
    match current_depth {
        1 => {
            for board in boards {
                let score = scoring_agent.score(&board);
                forest.new_child(parent_id, Some(board.clone()), score);
                level(
                    forest,
                    board,
                    parent_id,
                    !current_player_is_ai,
                    current_depth + 1,
                    scoring_agent,
                    depth_level,
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

impl<T: ScoringAgent> MinMaxAI<T> {
    fn grow_forests(&mut self) {
        let current_player_is_ai = false;
        let mut boards = vec![];
        if self.state_trees.is_empty() {
            boards = super::generate_options_for_current_board(
                &self.current_state,
                current_player_is_ai,
            );
            for board in boards {
                let score = self.scoring_agent.score(&board);
                self.state_trees
                    .push(Forest::new(Node::new(0, Some(board), score)));
            }
        }
        for forest in self.state_trees.iter_mut() {
            println!("Forest creation started!");
            level(
                forest,
                forest.0[0].get_data().unwrap(),
                0,
                !current_player_is_ai,
                1,
                self.scoring_agent,
                self.depth_level,
            );
            println!("Forest creation finished!");
        }
    }
}

impl<T: ScoringAgent> AI for MinMaxAI<T>
where
    T: Sync,
{
    fn make_move(&mut self, board: crate::board::layout::Board) -> crate::board::layout::Board {
        let mut it = self.state_trees.iter_mut();
        let correct_forest = loop {
            if let Some(forest) = it.next() {
                if forest.0[0].get_data().as_ref() == Some(&board) {
                    break forest;
                }
            }
        };

        // How do I find the correct path??
        correct_forest.0[correct_forest.0[0].children[0]]
            .get_data()
            .unwrap()
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
