use rand;
use rand::prelude::ThreadRng;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use rayon::prelude::*;
use std::fmt::Debug;

pub trait Evolutionable {
    fn mutate(&self, mutations: u32, rng: &mut ThreadRng) -> Self;
}

pub trait Evaluator<T: Evolutionable> {
    fn evaluate(&self, sol: &T) -> i32;
}

pub struct GeneticAlgorithm<
    T: Evolutionable + Clone + Debug + Send + Sync,
    E: Evaluator<T> + Default + Send + Sync,
> {
    evaluator: E,
    population: Vec<T>,
    mutations_per_generation: u32,
    elitism: f32,
    generation: u32,
    pop_size: usize,
}

impl<T: Evolutionable + Clone + Debug + Send + Sync, E: Evaluator<T> + Default + Send + Sync>
    GeneticAlgorithm<T, E>
{
    pub fn new(
        pop_size: usize,
        elitism: f32,
        mutations_per_generation: u32,
        evaluator: E,
        solution_prototype: &T,
    ) -> Self {
        GeneticAlgorithm {
            evaluator,
            pop_size,
            elitism,
            mutations_per_generation,
            population: {
                let mut v = Vec::new();
                for _ in 0..pop_size {
                    v.push(solution_prototype.clone())
                }
                v
            },
            generation: 0,
        }
    }

    pub fn next_generation(&mut self, rng: &mut ThreadRng) {
        let eval = std::mem::replace(&mut self.evaluator, E::default());
        let population = std::mem::replace(&mut self.population, Vec::new());
        let mut t = population
            .into_par_iter()
            .map(|x| {
                let score = eval.evaluate(&x);
                (x, score)
            })
            .collect::<Vec<_>>();
        t.sort_by(|x, y| y.1.cmp(&x.1));
        self.population = t.into_iter().map(|(obj, _)| obj).collect();
        self.evaluator = eval;

        self.population
            .drain(((self.pop_size as f32 * self.elitism) as usize)..);
        let removed = self.pop_size - self.population.len();
        let left = self.population.len();
        for _ in 0..removed {
            self.population.push(
                self.population[(0..left).choose(rng).unwrap()]
                    .mutate(self.mutations_per_generation, rng),
            );
        }
        self.generation += 1;
    }

    pub fn get_best(&self) -> Option<&T> {
        self.population.first()
    }

    pub fn evaluator(&self) -> &E {
        &self.evaluator
    }
}
