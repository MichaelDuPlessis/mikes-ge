use rand::{self, Rng};

use crate::grammer::Grammer;

pub struct GE {
    size: usize,
    weights: (f32, f32, f32),
    min_len: usize,
    max_len: usize,
    generations: usize,
    runs: usize,
    population: Vec<Vec<u8>>,
}

impl GE {
    pub fn new(
        size: usize,
        weights: (f32, f32, f32),
        min_len: usize,
        max_len: usize,
        generations: usize,
        runs: usize,
    ) -> Self {
        assert!(min_len < max_len);

        let population = Self::generate_initial_population(size, min_len, max_len);
        Self {
            size,
            weights,
            min_len,
            max_len,
            generations,
            runs,
            population,
        }
    }

    fn generate_individual(min_len: usize, max_len: usize) -> Vec<u8> {
        let len = rand::thread_rng().gen_range(min_len..=max_len);
        let individual = (0..len).map(|_| rand::random::<u8>()).collect();
        individual
    }

    fn generate_initial_population(size: usize, min_len: usize, max_len: usize) -> Vec<Vec<u8>> {
        let mut population = Vec::with_capacity(size);
        for _ in 0..size {
            population.push(Self::generate_individual(min_len, max_len));
        }
        population
    }

    pub fn start<I, O, F: Fn(I) -> O>(&self, grammer: impl Grammer<Input = I, Output = O>) -> F {
        todo!()
    }
}
