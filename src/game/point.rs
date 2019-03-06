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
use serde_derive::Deserialize;

use super::{Dir, SafetyIndex, Snake, State};

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    /// Returns the manhattan distance between self and p
    pub fn manhattan(self, p: Self) -> u32 {
        #![allow(clippy::pedantic)]
        ((self.x - p.x).abs() + (self.y - p.y).abs()) as u32
    }

    /// Returns the direction from this point to point p
    pub fn dir_to(self, p: Self) -> Option<Dir> {
        if p.y - self.y > 0 {
            return Some(Dir::Down);
        } else if p.y - self.y < 0 {
            return Some(Dir::Up);
        } else if p.x - self.x > 0 {
            return Some(Dir::Right);
        } else if p.x - self.x < 0 {
            return Some(Dir::Left);
        }
        None
    }

    /// Returns the 4 adjacent points to self
    pub fn orthogonal(self) -> [Self; 4] {
        [
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }

    /// IsValid is a version of safety_index that is meant to
    /// be run on states where the snakes have already updated,
    /// not for future states.
    ///
    /// TODO: Write a unit test for this funciton
    pub fn is_valid(self, s: &Snake, st: &State) -> bool {
        for (id, snake) in &st.board.snakes {
            if self == snake.body[0] && *id != s.id {
                return snake.body.len() < s.body.len();
            }

            if snake.body.iter().skip(1).any(|p| *p == self) {
                return false;
            }
        }

        self.in_bounds(st)
    }

    /// Returns the safety index of self.
    ///
    /// Safe: Empty point, in bounds, no snakes adjacent
    /// Risky: Empty point, in bounds, larger snake adjacent
    /// Unsafe: Occupied or OOB
    pub fn safety_index(self, s: &Snake, st: &State) -> SafetyIndex {
        for snake in &st.board.snakes {
            if snake.1.body.iter().any(|p| *p == self) {
                let len = snake.1.body.len();

                if self != snake.1.body[len - 1] || snake.1.body[len - 1] == snake.1.body[len - 2] {
                    return SafetyIndex::Unsafe;
                }
            }

            let contains = self
                .orthogonal()
                .iter()
                .any(|p| p.y == snake.1.body[0].y && p.x == snake.1.body[0].x);
            if snake.0 != &s.id && contains && snake.1.body.len() >= s.body.len() {
                return SafetyIndex::Risky;
            }
        }

        if self.in_bounds(st) {
            SafetyIndex::Safe
        } else {
            SafetyIndex::Unsafe
        }
    }

    /// Returns whether the point is inside the board
    pub fn in_bounds(self, st: &State) -> bool {
        self.x < st.board.width && self.x >= 0 && self.y < st.board.height && self.y >= 0
    }
}

// Implement methods for A*
impl Point {
    /// Returns the successors to self. Used for A*
    pub fn successors(self, s: &Snake, st: &State) -> Vec<(Self, u32)> {
        vec![
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
        ]
        .into_iter()
        .filter_map(|p| match p.safety_index(s, st) {
            SafetyIndex::Safe | SafetyIndex::Risky => Some((p, 1)),
            _ => None,
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::load_sample_data;
    use super::super::Dir;
    use super::super::SafetyIndex;
    use super::*;

    #[test]
    fn test_manhattan() {
        let points = vec![
            Point { x: 1, y: 1 },
            Point { x: 10, y: 10 },
            Point { x: 3, y: 4 },
            Point { x: 6, y: 5 },
            Point { x: 20, y: 0 },
            Point { x: 19, y: 4 },
            Point { x: 6, y: 22 },
            Point { x: 10, y: 15 },
        ];

        let dists = vec![18, 13, 4, 19, 5, 31, 11, 23];

        for (i, point) in points.iter().enumerate() {
            if i < points.len() - 1 {
                assert_eq!(point.manhattan(points[i + 1]), dists[i]);
            } else {
                assert_eq!(point.manhattan(points[0]), dists[i]);
            }
        }
    }

    #[test]
    fn test_dir_to() {
        let points = vec![
            Point { x: 1, y: 1 },
            Point { x: 10, y: 10 },
            Point { x: 3, y: 4 },
            Point { x: 6, y: 5 },
            Point { x: 20, y: 5 },
            Point { x: 19, y: 5 },
            Point { x: 6, y: 22 },
            Point { x: 10, y: 1 },
        ];

        let dirs = vec![
            Dir::Down,
            Dir::Up,
            Dir::Down,
            Dir::Right,
            Dir::Left,
            Dir::Down,
            Dir::Up,
            Dir::Left,
        ];

        for (i, point) in points.iter().enumerate() {
            if i < points.len() - 1 {
                assert_eq!(point.dir_to(points[i + 1]), Some(dirs[i]));
            } else {
                assert_eq!(point.dir_to(points[0]), Some(dirs[i]));
            }
        }

        let zero_dir_point = Point { x: 5, y: 5 };
        assert_eq!(zero_dir_point.dir_to(zero_dir_point), None);
    }

    #[test]
    fn test_orthogonal() {
        assert_eq!(
            Point { x: 5, y: 5 }.orthogonal(),
            [
                Point { x: 5, y: 4 },
                Point { x: 5, y: 6 },
                Point { x: 4, y: 5 },
                Point { x: 6, y: 5 },
            ]
        );
    }

    #[test]
    fn test_safety_index() {
        let datas = load_sample_data();
        assert_eq!(
            Point { x: 2, y: 8 }.safety_index(&datas[0].0, &datas[0].1),
            SafetyIndex::Safe
        );
        assert_eq!(
            Point { x: 3, y: 7 }.safety_index(&datas[0].0, &datas[0].1),
            SafetyIndex::Risky
        );
        assert_eq!(
            Point { x: 4, y: 7 }.safety_index(&datas[0].0, &datas[0].1),
            SafetyIndex::Unsafe
        );
        assert_eq!(
            Point { x: -1, y: -1 }.safety_index(&datas[0].0, &datas[0].1),
            SafetyIndex::Unsafe
        );
        assert_eq!(
            Point { x: 10, y: 2 }.safety_index(&datas[5].0, &datas[5].1),
            SafetyIndex::Unsafe
        );
    }

    #[test]
    fn test_in_bounds() {
        let datas = load_sample_data();

        assert_eq!(Point { x: -1, y: 3 }.in_bounds(&datas[0].1), false);
        assert_eq!(Point { x: 1, y: -3 }.in_bounds(&datas[0].1), false);
        assert_eq!(Point { x: -1, y: -3 }.in_bounds(&datas[0].1), false);
        assert_eq!(Point { x: 1, y: 3 }.in_bounds(&datas[0].1), true);
    }

    #[test]
    fn test_successors() {
        let datas = load_sample_data();

        assert_eq!(
            Point { x: 3, y: 8 }.successors(&datas[0].0, &datas[0].1),
            vec![
                (Point { x: 3, y: 7 }, 1),
                (Point { x: 2, y: 8 }, 1),
                (Point { x: 4, y: 8 }, 1),
            ]
        );
        assert_eq!(
            Point { x: 1, y: 3 }.successors(&datas[1].0, &datas[1].1),
            vec![
                (Point { x: 1, y: 2 }, 1),
                (Point { x: 1, y: 4 }, 1),
                (Point { x: 2, y: 3 }, 1),
            ]
        );
    }
}
