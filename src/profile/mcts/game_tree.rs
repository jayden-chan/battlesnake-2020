use crate::game::{Dir, SafetyIndex, Snake, State};
use crate::simulator::{process_step, Future};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32;

use log::{debug, info};
use rand::prelude::*;

#[derive(Clone, Debug)]
struct Node {
    parent: Option<usize>,
    children: [Option<usize>; 4],
    score: usize,
    sim_count: usize,
    state: State,
    future: Option<Future>,
    is_self_node: bool,
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

#[derive(Clone)]
pub struct GameTree {
    inner_vec: Vec<Node>,
    self_id: String,
    enemy_id: String,
}

impl GameTree {
    pub fn new(state: State, self_id: String, enemy_id: String) -> Self {
        Self {
            inner_vec: vec![Node {
                parent: None,
                children: [None, None, None, None],
                score: 0,
                sim_count: 0,
                future: None,
                state,
                is_self_node: false,
            }],
            self_id,
            enemy_id,
        }
    }

    pub fn root_child_scores(&self) -> Vec<(usize, usize)> {
        self.inner_vec[0]
            .children
            .iter()
            .filter_map(|i| match i {
                Some(e) => Some((self.inner_vec[*e].sim_count, *e)),
                None => None,
            })
            .collect::<Vec<(usize, usize)>>()
    }

    pub fn get_best_move(&self, scores: Vec<(usize, usize)>) -> Dir {
        let mut scores = scores;
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
        self.inner_vec[node_id].children[0].is_none()
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

        debug!("selecting {}", scores[0].1);

        scores[0].1
    }

    fn get_rollout_score(&mut self, node_id: usize) -> usize {
        let curr_future = self.inner_vec[node_id].future;

        match curr_future {
            Some(f) if f.finished => {
                if f.alive {
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => {
                let mut tmp_state = self.inner_vec[node_id].state.clone();
                let mut rng = rand::thread_rng();

                if self.inner_vec[node_id].is_self_node {
                    let mut moves = HashMap::new();
                    let enemy_snake =
                        tmp_state.board.snakes.get(&self.enemy_id).unwrap();

                    moves.insert(
                        self.enemy_id.clone(),
                        *get_snake_successors(enemy_snake, &tmp_state, false)
                            .choose(&mut rng)
                            .unwrap_or(&Dir::Up),
                    );

                    let tmp_future =
                        process_step(&mut tmp_state, &self.self_id, &moves);

                    if tmp_future.finished {
                        if tmp_future.alive {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }

                loop {
                    let moves = get_rollout_moves(&tmp_state, &mut rng);
                    let future =
                        process_step(&mut tmp_state, &self.self_id, &moves);

                    if future.finished {
                        if future.alive {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }
    }

    pub fn rollout(&mut self, node_id: usize) {
        let mut curr = node_id;

        let score = self.get_rollout_score(node_id);

        // Back-propogate the result of the rollout
        loop {
            self.inner_vec[curr].score += score;
            self.inner_vec[curr].sim_count += 1;

            if self.inner_vec[curr].parent.is_none() {
                break;
            }
            curr = self.inner_vec[curr].parent.unwrap();
        }
    }

    pub fn expand(&mut self, node_id: usize) -> Option<usize> {
        match self.inner_vec[node_id].future {
            Some(future) if future.finished => {
                return None;
            }
            _ => {}
        };

        let curr_state = self.inner_vec[node_id].state.clone();
        let curr_idx = self.inner_vec.len();
        let is_self_node = !self.inner_vec[node_id].is_self_node;

        let node_snake_id = if is_self_node {
            self.self_id.clone()
        } else {
            self.enemy_id.clone()
        };

        let node_snake = curr_state.board.snakes.get(&node_snake_id).unwrap();

        let successors =
            get_snake_successors(&node_snake, &curr_state, is_self_node);

        for (idx, dir) in successors.iter().enumerate() {
            self.create_node(
                node_id,
                &curr_state,
                *dir,
                node_snake_id.clone(),
                is_self_node,
            );
            self.inner_vec[node_id].children[idx] = Some(curr_idx + idx);
        }

        if is_self_node {
            let mut term_idx = successors.len();
            for p in node_snake.body[0].orthogonal().iter() {
                if p.safety_index(&node_snake, &curr_state)
                    == SafetyIndex::Risky
                {
                    self.create_terminal_node(node_id, &curr_state, 0);
                    self.inner_vec[node_id].children[term_idx] =
                        Some(curr_idx + term_idx);
                    term_idx += 1;
                }
            }
        }

        return self.inner_vec[node_id].children[0];
    }

    fn create_terminal_node(
        &mut self,
        parent_id: usize,
        st: &State,
        score: usize,
    ) {
        let new_state = st.clone();

        let future = Future {
            alive: false,
            finished: true,
            dead_snakes: 0,
            foods: 0,
            enemy_foods: 0,
            dir: Dir::Up,
        };

        self.inner_vec.push(Node {
            parent: Some(parent_id),
            children: [None, None, None, None],
            sim_count: 0,
            state: new_state,
            future: Some(future),
            is_self_node: true,
            score,
        });
    }

    fn create_node(
        &mut self,
        parent_id: usize,
        st: &State,
        node_move: Dir,
        node_snake_id: String,
        is_self_node: bool,
    ) {
        let mut new_state = st.clone();
        let mut moves = HashMap::new();
        moves.insert(node_snake_id.clone(), node_move);
        let future = process_step(&mut new_state, &self.self_id, &moves);

        self.inner_vec.push(Node {
            parent: Some(parent_id),
            children: [None, None, None, None],
            score: 0,
            sim_count: 0,
            state: new_state,
            future: Some(future),
            is_self_node,
        });
    }
}

fn get_rollout_moves(st: &State, rng: &mut ThreadRng) -> HashMap<String, Dir> {
    let mut dirs = HashMap::<String, Dir>::with_capacity(st.board.snakes.len());
    for (id, s) in &st.board.snakes {
        dirs.insert(
            id.to_string(),
            *get_snake_successors(s, st, false)
                .choose(rng)
                .unwrap_or(&Dir::Up),
        );
    }

    dirs
}

fn get_snake_successors(s: &Snake, st: &State, avoid_risky: bool) -> Vec<Dir> {
    s.body[0]
        .orthogonal()
        .iter()
        .filter_map(|e| match e.safety_index(&s, &st) {
            SafetyIndex::Safe => s.body[0].dir_to(*e),
            SafetyIndex::Risky if !avoid_risky => s.body[0].dir_to(*e),
            _ => None,
        })
        .collect::<Vec<Dir>>()
}
