use crate::genetic::{Evaluator, Evolutionable};
use crate::maze::TileState::Empty;
use ndarray::{arr2, Array2};
use pathfinding::prelude::astar;
use rand;
use rand::seq::IteratorRandom;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TileState {
    Full,
    Empty,
}

impl Default for TileState {
    fn default() -> Self {
        TileState::Empty
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Maze {
    data: Array2<TileState>,
}

impl Maze {
    pub fn new_empty(size: (usize, usize)) -> Self {
        Maze {
            data: Array2::default(size),
        }
    }

    pub fn new_from(data: Array2<TileState>) -> Self {
        Maze { data }
    }

    pub fn size(&self) -> (usize, usize) {
        self.data.dim()
    }

    pub fn at(&self, x: usize, y: usize) -> Option<TileState> {
        self.data.get((x, y)).copied()
    }

    pub fn in_borders(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.size().0 as i32 && y < self.size().1 as i32
    }

    pub fn find_path(&self, start: (i32, i32), end: (i32, i32)) -> Option<(Vec<(i32, i32)>, i32)> {
        astar(
            &start,
            |&pos| {
                vec![
                    (pos.0 - 1, pos.1),
                    (pos.0 + 1, pos.1),
                    (pos.0, pos.1 - 1),
                    (pos.0, pos.1 + 1),
                ]
                .into_iter()
                .filter(|&(x, y)| {
                    self.in_borders(x, y) && self.data[[x as usize, y as usize]] == Empty
                })
                .map(|p| (p, 1))
            },
            |pos| end.0 - pos.0 + end.1 - pos.1,
            |&pos| pos == end,
        )
    }
}

impl Evolutionable for Maze {
    fn mutate(&self, mutations: u32, rng: &mut rand::prelude::ThreadRng) -> Self {
        let mut offspring = self.clone();

        let size = self.data.dim();
        for _ in 0..mutations {
            let rand_ind = [
                (0..size.0).choose(rng).unwrap(),
                (0..size.1).choose(rng).unwrap(),
            ];
            offspring.data[rand_ind] = match self.data[rand_ind] {
                TileState::Full => TileState::Empty,
                TileState::Empty => TileState::Full,
            };
        }
        offspring
    }
}

#[derive(Clone, Default)]
pub struct MazeEval {
    start: (i32, i32),
    end: (i32, i32),
}

impl MazeEval {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        MazeEval { start, end }
    }
}

impl Evaluator<Maze> for MazeEval {
    fn evaluate(&self, sol: &Maze) -> i32 {
        if sol
            .at(self.start.0 as usize, self.start.1 as usize)
            .unwrap()
            == TileState::Full
        {
            return -100000;
        }
        let mut prettiness_score = 0;
        for x in 0..sol.size().0 {
            for y in 0..sol.size().1 {
                if sol.at(x, y).unwrap() == TileState::Full {
                    prettiness_score -= 1;
                    let (x, y) = (x as i32, y as i32);
                    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .iter()
                        .filter(|&&(x, y)| sol.in_borders(x, y))
                        .for_each(|&(x, y)| {
                            if sol.at(x as usize, y as usize).unwrap() == TileState::Full {
                                prettiness_score += 1;
                            }
                        });
                }
            }
        }

        sol.find_path(self.start, self.end)
            .map_or(-100000, |(_, cost)| cost * 50 + prettiness_score)
    }
}
