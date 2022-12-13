use super::{ScoringAgent, AI};
use crate::board;
use crate::board::layout::Board;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::JoinHandle;

pub struct MinMaxAI<T: ScoringAgent + 'static> {
    state_tree: Arc<(Mutex<Vec<Forest>>, Condvar)>,
    scoring_agent: T,
    current_state: Board,
    depth_level: u32,
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
struct Node {
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    data: Option<Board>,
    score: u32,
}

impl Node {
    fn new(id: usize, data: Option<Board>, score: u32) -> Self {
        Self {
            id,
            parent: None,
            children: vec![],
            data,
            score,
        }
    }

    fn new_child(id: usize, parent_id: usize, data: Option<Board>, score: u32) -> Self {
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
struct Forest(Vec<Node>);

impl Forest {
    fn new(root_node: Node) -> Self {
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

    fn new_child(&mut self, parent_id: usize, data: Option<Board>, score: u32) -> usize {
        let child = Node::new_child(self.0.len(), parent_id, data, score);
        self.0[parent_id].children.push(child.id);
        self.0.push(child);
        return child.id;
    }
}

impl<T: ScoringAgent> MinMaxAI<T> {
    pub fn new(scoring_agent: T, board: Board, first_move: bool) -> Self {
        let min_max = if first_move {
            Self {
                current_state: board,
                state_tree: Arc::new((
                    Mutex::new(vec![Forest::new(Node::new(0, Some(board), 0))]),
                    Condvar::new(),
                )),
                scoring_agent,
                depth_level: 5,
            }
        } else {
            Self {
                current_state: board,
                state_tree: Arc::new((Mutex::new(vec![]), Condvar::new())),
                scoring_agent,
                depth_level: 5,
            }
        };
        std::thread::spawn(move || loop {
            min_max.grow_forests();
            min_max.state_tree.1.notify_one();
            min_max
                .state_tree
                .1
                .wait(min_max.state_tree.0.lock().unwrap());
        });
        min_max
    }

    fn level(
        &self,
        forest: &mut Forest,
        board: Board,
        parent_id: usize,
        current_player_is_ai: bool,
        current_depth: u32,
    ) {
        let boards = super::generate_options_for_current_board(&board, current_player_is_ai);
        match current_depth {
            1 => {
                for board in boards {
                    let score = self.scoring_agent.score(&board);
                    forest.new_child(parent_id, Some(board.clone()), score);
                    self.level(
                        forest,
                        board,
                        parent_id,
                        !current_player_is_ai,
                        current_depth + 1,
                    );
                }
            }
            depth if depth < self.depth_level => {
                for board in boards {
                    let score = self.scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                    self.level(forest, board, parent_id, !current_player_is_ai, depth + 1);
                }
            }
            depth => {
                for board in boards {
                    let score = self.scoring_agent.score(&board);
                    forest.new_child(parent_id, None, score);
                }
            }
        }
    }

    fn grow_forests(&mut self) {
        let state_trees = &mut self.state_tree.0.lock().unwrap();
        let current_player_is_ai = false;
        let mut boards = vec![];
        if state_trees.is_empty() {
            boards = super::generate_options_for_current_board(
                &self.current_state,
                current_player_is_ai,
            );
            for board in boards {
                let score = self.scoring_agent.score(&board);
                state_trees.push(Forest::new(Node::new(0, Some(board), score)));
            }
        }
        for forest in state_trees.iter_mut() {
            self.level(
                forest,
                forest.0[0].get_data().unwrap(),
                0,
                !current_player_is_ai,
                1,
            );
        }
    }
}

impl<T: ScoringAgent> AI for MinMaxAI<T>
where
    T: Sync,
{
    fn make_move(&mut self, board: crate::board::layout::Board) -> crate::board::layout::Board {
        let state_trees = &mut self.state_tree.0.lock().unwrap();
        let it = state_trees.iter_mut();
        let correct_forest = loop {
            let forest = it.next();
            if forest
                .expect("There was no forest with that specific board!!! Something is wrong!")
                .0[0]
                .get_data()
                .unwrap()
                == board
            {
                break forest.unwrap();
            }
        };

        // How do I find the correct path??
        todo!()
    }
}

// General idea:
// - have an engine which works on separate thread
// - on every new game state (after user chooses a move), lock mutex, update board, notify thread to wake up
// - when thread wakes up, it locks the mutex and calculates X next possible states, when finished -> wait for wakeup
// Is this possible to do with these since Min max should theoretically cut off some branches? (Research whether the cut off ones are possible at all)
