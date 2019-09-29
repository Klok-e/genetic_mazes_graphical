use genetic_mazes_lib::{Evaluator, Evolutionable, GeneticAlgorithm};
use genetic_mazes_lib::{Maze, MazeEval, TileState};
use ndarray::arr2;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;

#[test]
fn maze_mutate() {
    let mut rng = rand::thread_rng();
    let maze = Maze::new_empty((5, 5));
    assert_ne!(maze.mutate(1, &mut rng), maze);
}

#[test]
fn maze_score() {
    use TileState::{Empty as E, Full as F};

    let maze = Maze::new_from(arr2(&[
        [E, E, E, E, E],
        [E, F, E, E, E],
        [E, F, E, E, E],
        [E, F, F, F, F],
        [E, E, E, E, E],
    ]));
    assert_eq!(MazeEval::new((0, 0), (4, 4)).evaluate(&maze), 8);

    let maze = Maze::new_from(arr2(&[
        [E, E, E, E, E],
        [E, F, E, E, E],
        [E, F, E, E, E],
        [E, F, E, F, F],
        [E, E, E, F, E],
        [E, E, E, E, E],
    ]));
    assert_eq!(MazeEval::new((0, 0), (5, 4)).evaluate(&maze), 9);
}

#[test]
fn genetic_general() {
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct ToNum(i32);
    impl Evolutionable for ToNum {
        fn mutate(&self, muts: u32, rng: &mut ThreadRng) -> Self {
            let mut offspr = ToNum { 0: self.0 };
            for _ in 0..muts {
                offspr = ToNum {
                    0: offspr.0 + *([-1, 1]).choose(rng).unwrap(),
                };
            }
            offspr
        }
    }
    #[derive(Default)]
    struct NumEval(i32);
    impl Evaluator<ToNum> for NumEval {
        fn evaluate(&self, sol: &ToNum) -> i32 {
            -(self.0 - sol.0).abs()
        }
    }
    let mut rng = rand::thread_rng();
    let mut alg = GeneticAlgorithm::new(20, 0.2, 5, NumEval { 0: 100 }, &ToNum { 0: 0 });
    for _ in 0..100 {
        alg.next_generation(&mut rng);
    }
    assert_eq!(alg.get_best().unwrap(), &ToNum { 0: 100 });
}
