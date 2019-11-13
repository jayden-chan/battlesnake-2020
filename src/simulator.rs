use std::collections::{HashMap, HashSet};

use crate::game::{Dir, Point, State};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Future {
    /// Whether the protagonist snake is still alive
    pub alive: bool,
    /// Whether the protagonist snake has won
    pub finished: bool,
    /// The number of snakes that have died
    pub dead_snakes: u16,
    /// The number of foods collected by the protagonist
    pub foods: u16,
    /// The number of foods collected by enemies
    pub enemy_foods: u16,
    /// The starting direction of the future
    pub dir: Dir,
}

pub fn process_step(
    st: &mut State,
    self_id: &str,
    moves: &HashMap<String, Dir>,
) -> Future {
    let mut tmp_future = Future {
        alive: true,
        finished: false,
        dead_snakes: 0,
        foods: 0,
        enemy_foods: 0,
        dir: Dir::Up,
    };

    let mut results = HashMap::<&str, Point>::with_capacity(moves.len());
    let mut eaten_foods = HashSet::new();

    for (id, dir) in moves {
        if *id == self_id {
            tmp_future.dir = *dir;
        }

        let snake = st.board.snakes.get_mut(id).unwrap();
        let (head, food_eaten) = snake.update_from_move(*dir, &st.board.food);

        if let Some(p) = food_eaten {
            if *id == self_id {
                tmp_future.foods += 1;
            } else {
                tmp_future.enemy_foods += 1;
            }

            // self.st.board.food.remove(&p);
            eaten_foods.insert(p);
        }

        results.insert(id, head);
    }

    for food in &eaten_foods {
        st.board.food.remove(&food);
    }

    let mut to_remove = Vec::new();

    for (id, head) in results {
        let snake = st.board.snakes.get(id).unwrap();

        if !head.is_valid(snake, &st) || snake.health == 0 {
            if id == self_id {
                tmp_future.alive = false;
                tmp_future.finished = true;
            } else {
                tmp_future.dead_snakes += 1;
                to_remove.push(id);
            }
        }
    }

    for id in &to_remove {
        st.board.snakes.remove(*id);
    }

    if !to_remove.is_empty() && st.board.snakes.len() == 1 {
        tmp_future.finished = true;
    }

    tmp_future
}
