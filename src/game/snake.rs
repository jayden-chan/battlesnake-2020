/*
 * Copyright (C) 2019 Jayden Chan. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 *
 */
use hashbrown::HashSet;
use serde_derive::Deserialize;

use super::{Dir, Point, SafetyIndex, State};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Snake {
    pub id: String,
    pub health: u8,
    pub body: Vec<Point>,
}

impl Snake {
    /// Returns the location of the nearest food to self
    pub fn nearest_food(&self, st: &State) -> Option<Point> {
        let mut nearest_dist = 99;
        let mut nearest_food = None;

        for food in &st.board.food {
            let dist = self.body[0].manhattan(*food);
            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_food = Some(*food)
            }
        }

        nearest_food
    }

    /// Returns the location of the nearest snake to self
    pub fn nearest_snake<'a>(&self, st: &'a State) -> Option<&'a Self> {
        let mut nearest_dist = 99;
        let mut nearest_snake = None;

        for (id, snake) in &st.board.snakes {
            if self.id != *id {
                let dist = self.body[0].manhattan(snake.body[0]);
                if dist < nearest_dist {
                    nearest_dist = dist;
                    nearest_snake = Some(snake)
                }
            }
        }

        nearest_snake
    }

    /// Finds a safe space to move to. If there are no safe
    /// spaces this function defaults to "up"
    pub fn find_safe_move(&self, st: &State) -> Dir {
        let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        let levels = [SafetyIndex::Safe, SafetyIndex::Risky];
        let orthogonal = self.body[0].orthogonal();

        for level in &levels {
            for (i, dir) in dirs.iter().enumerate() {
                if orthogonal[i].safety_index(&self, st) == *level {
                    return *dir;
                }
            }
        }

        Dir::Up
    }

    /// Updates the snake's body and health based on the provided move
    pub fn update_from_move(&mut self, dir: Dir, food: &HashSet<Point>) -> (Point, Option<Point>) {
        let collected = dir.will_collect_food(self, food);

        let new_point = match dir {
            Dir::Up => Point {
                x: self.body[0].x,
                y: self.body[0].y - 1,
            },
            Dir::Down => Point {
                x: self.body[0].x,
                y: self.body[0].y + 1,
            },
            Dir::Left => Point {
                x: self.body[0].x - 1,
                y: self.body[0].y,
            },
            Dir::Right => Point {
                x: self.body[0].x + 1,
                y: self.body[0].y,
            },
        };

        self.body.insert(0, new_point);
        self.body.pop();

        if collected {
            self.health = 100;

            if let Some(p) = self.body.last() {
                self.body.push(*p);
            }

            (new_point, Some(new_point))
        } else {
            self.health -= 1;
            (new_point, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::load_sample_data;
    use super::*;

    const SELF_ID: &str = "2d397b8c-8b3f-416d-bb16-6bc85ab3226e";
    const SBOT_ID: &str = "0633b850-fa2b-4165-97d4-b88cf3acfe7f";
    const ALEX_ID: &str = "4e073745-ba79-4764-8c6c-388dd7b86943";

    #[test]
    fn test_nearest_food() {
        let datas = load_sample_data();

        assert_eq!(
            datas[0].1.board.snakes[SELF_ID].nearest_food(&datas[0].1),
            Some(Point { x: 3, y: 7 })
        );
        assert_eq!(
            datas[0].1.board.snakes[SBOT_ID].nearest_food(&datas[0].1),
            Some(Point { x: 11, y: 1 })
        );
        assert_eq!(
            datas[0].1.board.snakes[ALEX_ID].nearest_food(&datas[0].1),
            Some(Point { x: 3, y: 7 })
        );
    }

    #[test]
    fn test_find_safe_move() {
        let datas = load_sample_data();

        assert_eq!(
            (&datas[0].1.board.snakes[SELF_ID]).find_safe_move(&datas[0].1),
            Dir::Left
        );
        assert_eq!(
            (&datas[0].1.board.snakes[SBOT_ID]).find_safe_move(&datas[0].1),
            Dir::Up
        );
        assert_eq!(
            (&datas[0].1.board.snakes[ALEX_ID]).find_safe_move(&datas[0].1),
            Dir::Up
        );
    }

    #[test]
    fn test_update_from_move() {
        let data = &mut load_sample_data()[0];

        let snake = &mut data.1.board.snakes.get_mut(SBOT_ID).unwrap();

        let point = snake.update_from_move(Dir::Right, &data.1.board.food);
        assert_eq!(point, (Point { x: 12, y: 2 }, None));
        assert_eq!(
            snake.body,
            [
            Point { x: 12, y: 2 },
            Point { x: 11, y: 2 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 9, y: 3 },
            Point { x: 8, y: 3 },
            Point { x: 7, y: 3 },
            Point { x: 7, y: 2 },
            Point { x: 8, y: 2 },
            ]
        );

        assert_eq!(snake.health, 58);

        let point = snake.update_from_move(Dir::Up, &data.1.board.food);
        assert_eq!(point, (Point { x: 12, y: 1 }, None));
        assert_eq!(
            snake.body,
            [
            Point { x: 12, y: 1 },
            Point { x: 12, y: 2 },
            Point { x: 11, y: 2 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 9, y: 3 },
            Point { x: 8, y: 3 },
            Point { x: 7, y: 3 },
            Point { x: 7, y: 2 },
            ]
        );

        assert_eq!(snake.health, 57);

        let point = snake.update_from_move(Dir::Left, &data.1.board.food);
        assert_eq!(point, (Point { x: 11, y: 1 }, Some(Point { x: 11, y: 1 })));
        assert_eq!(
            snake.body,
            [
            Point { x: 11, y: 1 },
            Point { x: 12, y: 1 },
            Point { x: 12, y: 2 },
            Point { x: 11, y: 2 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 9, y: 3 },
            Point { x: 8, y: 3 },
            Point { x: 7, y: 3 },
            Point { x: 7, y: 3 },
            ]
        );

        assert_eq!(snake.health, 100);
    }
}
