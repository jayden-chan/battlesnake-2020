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
use log::info;

use super::{Move, Point, SafetyIndex, Snake, State};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    /// Converts the direction to a move
    pub fn as_move(self) -> Move {
        match self {
            Dir::Up => Move { dir: "up" },
            Dir::Down => Move { dir: "down" },
            Dir::Left => Move { dir: "left" },
            Dir::Right => Move { dir: "right" },
        }
    }

    /// Resulting point returns the point that the direction
    /// points to from point p
    pub fn resulting_point(self, p: Point) -> Point {
        match self {
            Dir::Up => Point { x: p.x, y: p.y - 1 },
            Dir::Down => Point { x: p.x, y: p.y + 1 },
            Dir::Left => Point { x: p.x - 1, y: p.y },
            Dir::Right => Point { x: p.x + 1, y: p.y },
        }
    }

    /// Tests if the direction is safe to move to
    pub fn is_safety_index(self, s: &Snake, st: &State, se: &SafetyIndex) -> bool {
        let head = s.body[0];
        self.resulting_point(head).safety_index(s, st) == *se
    }

    /// Whether this move will cause the snake to collect food
    pub fn will_collect_food(self, s: &Snake, food: &HashSet<Point>) -> bool {
        let head = s.body[0];
        food.contains(&self.resulting_point(head))
    }

    /// This function tests to see if a move could result
    /// in the snake being corner-adjacent to another larger snake
    pub fn is_corner_risky(self, s: &Snake, st: &State) -> bool {
        let mut diagonal_points = Vec::with_capacity(2);
        let mut outer_points = Vec::with_capacity(4);
        let mut blocker_points = Vec::with_capacity(2);

        let head = s.body[0];

        match self {
            Dir::Up => {
                // verified
                diagonal_points.push(Point {
                    x: head.x - 1,
                    y: head.y - 2,
                });
                diagonal_points.push(Point {
                    x: head.x + 1,
                    y: head.y - 2,
                });

                outer_points.push(Point {
                    x: head.x - 2,
                    y: head.y - 2,
                });
                outer_points.push(Point {
                    x: head.x - 1,
                    y: head.y - 3,
                });
                outer_points.push(Point {
                    x: head.x + 1,
                    y: head.y - 3,
                });
                outer_points.push(Point {
                    x: head.x + 2,
                    y: head.y - 2,
                });

                blocker_points.push(Point {
                    x: head.x - 1,
                    y: head.y - 1,
                });
                blocker_points.push(Point {
                    x: head.x + 1,
                    y: head.y - 1,
                });
            }
            Dir::Down => {
                // verified
                diagonal_points.push(Point {
                    x: head.x + 1,
                    y: head.y + 2,
                });
                diagonal_points.push(Point {
                    x: head.x - 1,
                    y: head.y + 2,
                });

                outer_points.push(Point {
                    x: head.x + 2,
                    y: head.y + 2,
                });
                outer_points.push(Point {
                    x: head.x + 1,
                    y: head.y + 3,
                });
                outer_points.push(Point {
                    x: head.x - 1,
                    y: head.y + 3,
                });
                outer_points.push(Point {
                    x: head.x - 2,
                    y: head.y + 2,
                });

                blocker_points.push(Point {
                    x: head.x + 1,
                    y: head.y + 1,
                });
                blocker_points.push(Point {
                    x: head.x - 1,
                    y: head.y + 1,
                });
            }
            Dir::Left => {
                // verified
                diagonal_points.push(Point {
                    x: head.x - 2,
                    y: head.y + 1,
                });
                diagonal_points.push(Point {
                    x: head.x - 2,
                    y: head.y - 1,
                });

                outer_points.push(Point {
                    x: head.x - 2,
                    y: head.y + 2,
                });
                outer_points.push(Point {
                    x: head.x - 3,
                    y: head.y + 1,
                });
                outer_points.push(Point {
                    x: head.x - 3,
                    y: head.y - 1,
                });
                outer_points.push(Point {
                    x: head.x - 2,
                    y: head.y - 2,
                });

                blocker_points.push(Point {
                    x: head.x - 1,
                    y: head.y + 1,
                });
                blocker_points.push(Point {
                    x: head.x - 1,
                    y: head.y - 1,
                });
            }
            Dir::Right => {
                // verified
                diagonal_points.push(Point {
                    x: head.x + 2,
                    y: head.y - 1,
                });
                diagonal_points.push(Point {
                    x: head.x + 2,
                    y: head.y + 1,
                });

                outer_points.push(Point {
                    x: head.x + 2,
                    y: head.y - 2,
                });
                outer_points.push(Point {
                    x: head.x + 3,
                    y: head.y - 1,
                });
                outer_points.push(Point {
                    x: head.x + 3,
                    y: head.y + 1,
                });
                outer_points.push(Point {
                    x: head.x + 2,
                    y: head.y + 2,
                });

                blocker_points.push(Point {
                    x: head.x + 1,
                    y: head.y - 1,
                });
                blocker_points.push(Point {
                    x: head.x + 1,
                    y: head.y + 1,
                });
            }
        }

        for (_, snake) in &st.board.snakes {
            for point in &snake.body {
                diagonal_points.retain(|p| *p != *point)
            }
        }

        if diagonal_points.len() == 0 {
            return false;
        }

        for (id, snake) in &st.board.snakes {
            if *id == s.id {
                continue;
            }

            if snake.body.len() >= s.body.len() {
                if outer_points[0] == snake.body[0] || outer_points[1] == snake.body[0] {
                    info!("returning safety_index from corner adj");
                    return blocker_points[1].safety_index(s, st) == SafetyIndex::Unsafe;
                }

                if outer_points[2] == snake.body[0] || outer_points[3] == snake.body[0] {
                    info!("returning safety_index from corner adj");
                    return blocker_points[0].safety_index(s, st) == SafetyIndex::Unsafe;
                }
            }
        }

        false
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
    fn test_as_move() {
        assert_eq!(Dir::Up.as_move(), Move { dir: "up" });
        assert_eq!(Dir::Down.as_move(), Move { dir: "down" });
        assert_eq!(Dir::Left.as_move(), Move { dir: "left" });
        assert_eq!(Dir::Right.as_move(), Move { dir: "right" });
    }

    #[test]
    fn test_resulting_point() {
        assert_eq!(
            Dir::Up.resulting_point(Point { x: 10, y: 10 }),
            Point { x: 10, y: 9 }
        );
        assert_eq!(
            Dir::Down.resulting_point(Point { x: 10, y: 10 }),
            Point { x: 10, y: 11 }
        );
        assert_eq!(
            Dir::Left.resulting_point(Point { x: 10, y: 10 }),
            Point { x: 9, y: 10 }
        );
        assert_eq!(
            Dir::Right.resulting_point(Point { x: 10, y: 10 }),
            Point { x: 11, y: 10 }
        );
    }

    #[test]
    fn test_is_safety_index() {
        let datas = load_sample_data();
        let data = &datas[0];
        let data1 = &datas[5];

        assert_eq!(
            Dir::Up.is_safety_index(&data.0, &data.1, &SafetyIndex::Safe),
            false
        );
        assert_eq!(
            Dir::Down.is_safety_index(&data.0, &data.1, &SafetyIndex::Unsafe),
            true
        );
        assert_eq!(
            Dir::Left.is_safety_index(&data.0, &data.1, &SafetyIndex::Safe),
            true
        );
        assert_eq!(
            Dir::Right.is_safety_index(&data.0, &data.1, &SafetyIndex::Risky),
            true
        );
        assert_eq!(
            Dir::Up.is_safety_index(&data.1.board.snakes[ALEX_ID], &data.1, &SafetyIndex::Safe),
            true
        );
        assert_eq!(
            Dir::Down.is_safety_index(&data.1.board.snakes[ALEX_ID], &data.1, &SafetyIndex::Safe),
            true
        );
        assert_eq!(
            Dir::Left.is_safety_index(&data.1.board.snakes[ALEX_ID], &data.1, &SafetyIndex::Safe),
            true
        );
        assert_eq!(
            Dir::Right.is_safety_index(
                &data.1.board.snakes[ALEX_ID],
                &data.1,
                &SafetyIndex::Unsafe
            ),
            true
        );
        assert_eq!(
            Dir::Right.is_safety_index(
                &data1.1.board.snakes[SELF_ID],
                &data1.1,
                &SafetyIndex::Unsafe
            ),
            true
        );
    }

    #[test]
    fn test_will_collect_food() {
        let data = &load_sample_data()[0];

        assert_eq!(
            Dir::Up.will_collect_food(&data.1.board.snakes[SELF_ID], &data.1.board.food),
            true
        );
        assert_eq!(
            Dir::Down.will_collect_food(&data.1.board.snakes[SELF_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Left.will_collect_food(&data.1.board.snakes[SELF_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Right.will_collect_food(&data.1.board.snakes[SELF_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Up.will_collect_food(&data.1.board.snakes[ALEX_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Down.will_collect_food(&data.1.board.snakes[ALEX_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Left.will_collect_food(&data.1.board.snakes[ALEX_ID], &data.1.board.food),
            true
        );
        assert_eq!(
            Dir::Right.will_collect_food(&data.1.board.snakes[ALEX_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Up.will_collect_food(&data.1.board.snakes[SBOT_ID], &data.1.board.food),
            true
        );
        assert_eq!(
            Dir::Down.will_collect_food(&data.1.board.snakes[SBOT_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Left.will_collect_food(&data.1.board.snakes[SBOT_ID], &data.1.board.food),
            false
        );
        assert_eq!(
            Dir::Right.will_collect_food(&data.1.board.snakes[SBOT_ID], &data.1.board.food),
            false
        );
    }
}
