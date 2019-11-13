use crate::game::{Dir, SafetyIndex, Snake, State};
use crate::simulator::{process_step, Future};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32;

use rand::prelude::*;

struct Node {
    parent: Option<usize>,
    children: [Option<usize>; 3],
    score: usize,
    sim_count: usize,
    state: State,
}

impl Node {
    pub fn ucb_one(&self, N: usize) -> f32 {
        if self.sim_count == 0 {
            f32::MAX
        } else {
            (self.score as f32 / self.sim_count as f32)
                + 2.0 * f32::sqrt(f32::ln(N as f32) / self.sim_count as f32)
        }
    }
}

pub struct GameTree {
    inner_vec: Vec<Node>,
    self_id: String,
}

impl GameTree {
    pub fn new(state: State, self_id: String) -> Self {
        Self {
            inner_vec: vec![Node {
                parent: None,
                children: [None, None, None],
                score: 0,
                sim_count: 0,
                state,
            }],
            self_id,
        }
    }

    pub fn get_best_move(&self) -> Dir {
        let mut scores = self.inner_vec[0]
            .children
            .iter()
            .filter_map(|i| match i {
                Some(e) => Some((self.inner_vec[*e].sim_count, *e)),
                None => None,
            })
            .collect::<Vec<(usize, usize)>>();

        scores.sort_by(|a, b| {
            if a.0 > b.0 {
                Ordering::Less
            } else if a.0 < b.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let self_snake = self.inner_vec[scores[0].1]
            .state
            .board
            .snakes
            .get(&self.self_id)
            .unwrap();

        self_snake.body[1].dir_to(self_snake.body[0]).unwrap()
    }

    pub fn node_is_leaf(&self, node_id: usize) -> bool {
        self.inner_vec[node_id].children[0].is_some()
    }

    pub fn node_has_sims(&self, node_id: usize) -> bool {
        self.inner_vec[node_id].sim_count > 0
    }

    pub fn next_node(&self, node_id: usize) -> usize {
        let curr_node = &self.inner_vec[node_id];
        let children = curr_node.children;

        let N = self.inner_vec[0].sim_count;

        let mut scores = children
            .iter()
            .filter_map(|i| match i {
                Some(e) => Some((self.inner_vec[*e].ucb_one(N), *e)),
                None => None,
            })
            .collect::<Vec<(f32, usize)>>();

        scores.sort_by(|a, b| {
            if a.0 > b.0 {
                Ordering::Less
            } else if a.0 < b.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        scores[0].1
    }

    pub fn rollout(&mut self, node_id: usize) {
        let mut tmp_state = self.inner_vec[node_id].state.clone();

        let mut rng = rand::thread_rng();
        let score = loop {
            let moves = get_rollout_moves(&self.self_id, &tmp_state, &mut rng);
            let future = process_step(&mut tmp_state, &self.self_id, &moves);

            if future.finished {
                if future.alive {
                    break 1;
                } else {
                    break 0;
                }
            }
        };

        let mut curr = node_id;

        // Back-propogate the result of the rollout
        while self.inner_vec[curr].parent.is_some() {
            self.inner_vec[curr].score += score;
            self.inner_vec[curr].sim_count += 1;

            curr = self.inner_vec[curr].parent.unwrap();
        }
    }

    pub fn expand(&mut self, node_id: usize) -> Option<usize> {
        let curr_state = self.inner_vec[node_id].state.clone();
        let curr_idx = self.inner_vec.len();

        let self_snake = curr_state.board.snakes.get(&self.self_id).unwrap();
        let self_successors = get_snake_successors(&self_snake, &curr_state);

        let mut rng = rand::thread_rng();

        for (idx, dir) in self_successors.iter().enumerate() {
            self.create_node(node_id, &curr_state, *dir, &mut rng);
            self.inner_vec[node_id].children[idx] = Some(curr_idx + idx);
        }

        return match self_successors.len() {
            0 => None,
            _ => Some(curr_idx),
        };
    }

    fn create_node(
        &mut self,
        parent_id: usize,
        st: &State,
        self_move: Dir,
        rng: &mut ThreadRng,
    ) {
        let mut new_state = st.clone();
        let moves =
            get_expansion_moves(&self.self_id, self_move, &new_state, rng);
        process_step(&mut new_state, &self.self_id, &moves);

        self.inner_vec.push(Node {
            parent: Some(parent_id),
            children: [None, None, None],
            score: 0,
            sim_count: 0,
            state: new_state,
        });
    }
}

fn get_expansion_moves(
    self_id: &str,
    self_move: Dir,
    st: &State,
    rng: &mut ThreadRng,
) -> HashMap<String, Dir> {
    let mut dirs = HashMap::<String, Dir>::with_capacity(st.board.snakes.len());
    for (id, s) in &st.board.snakes {
        let dir = if *id == self_id {
            self_move
        } else {
            *get_snake_successors(s, st).choose(rng).unwrap_or(&Dir::Up)
        };

        dirs.insert(id.to_string(), dir);
    }

    dirs
}

fn get_rollout_moves(
    self_id: &str,
    st: &State,
    rng: &mut ThreadRng,
) -> HashMap<String, Dir> {
    let mut dirs = HashMap::<String, Dir>::with_capacity(st.board.snakes.len());
    for (id, s) in &st.board.snakes {
        dirs.insert(
            id.to_string(),
            *get_snake_successors(s, st).choose(rng).unwrap_or(&Dir::Up),
        );
    }

    dirs
}

fn get_snake_successors(s: &Snake, st: &State) -> Vec<Dir> {
    s.body[0]
        .orthogonal()
        .iter()
        .filter_map(|e| {
            if e.safety_index(&s, &st) != SafetyIndex::Unsafe {
                s.body[0].dir_to(*e)
            } else {
                None
            }
        })
        .collect::<Vec<Dir>>()
}
