use std::collections::HashMap;

use crate::grammer::{Distance, Grammer};
use rand::{self, seq::SliceRandom, Rng};

// my types
type Chromosome = Vec<u8>;

pub struct GE<I, O, G>
where
    G: Grammer<Input = I, Output = O>,
    O: Distance,
{
    size: usize,
    weights: (f32, f32, f32),
    min_len: usize,
    max_len: usize,
    generations: usize,
    tournament: usize, // tournament size
    runs: usize,
    train: Vec<(I, O)>,
    grammer: G,
    population: Vec<Chromosome>,
}

impl<I, O, G> GE<I, O, G>
where
    G: Grammer<Input = I, Output = O>,
    O: Distance,
{
    pub fn new(
        size: usize,
        weights: (f32, f32, f32),
        min_len: usize,
        max_len: usize,
        generations: usize,
        tournament: usize,
        runs: usize,
        train: Vec<(I, O)>,
        grammer: G,
    ) -> Self {
        assert!(min_len < max_len);
        assert!(weights.0 + weights.1 + weights.2 == 1.0);

        Self {
            size,
            weights,
            min_len,
            max_len,
            generations,
            tournament,
            runs,
            train,
            grammer,
            population: Vec::with_capacity(size),
        }
    }

    // ====================================================================
    // creating the initial population
    fn generate_individual(&mut self) {
        let len = rand::thread_rng().gen_range(self.min_len..=self.max_len);
        let individual = (0..len).map(|_| rand::random::<u8>()).collect();
        self.population.push(individual);
    }

    fn generate_initial_population(&mut self) {
        self.population.clear();
        for _ in 0..self.size {
            self.generate_individual();
        }
    }

    // ====================================================================
    // everything for creating a new population

    fn raw_fitness(&self, chromosome: &Chromosome) -> f64 {
        let individual = G::generate(&chromosome);
        self.train
            .iter()
            .map(|(input, expected)| (expected.distance(&individual.run(input))).abs())
            .sum()
    }

    // performs tournament selection on the population and returns the index of the chosen individual
    fn tournament_selection(&self, cache: &mut HashMap<&Chromosome, f64>) -> &Chromosome {
        self.population
            .choose_multiple(&mut rand::thread_rng(), self.tournament)
            .max_by(|c1, c2| {
                let f1 = if let Some(f) = cache.get(c1) {
                    *f
                } else {
                    let f = self.raw_fitness(c1);
                    cache.insert(c1, f);
                    f
                };
                let f2 = if let Some(f) = cache.get(c2) {
                    *f
                } else {
                    let f = self.raw_fitness(c2);
                    cache.insert(c2, f);
                    f
                };

                f1.total_cmp(&f2)
            })
            .unwrap()
    }

    fn generate_next_population(&mut self) {
        // TODO: change to with_capacity when a good strategy for figuring out what the capcity
        // will be
        let cache = HashMap::new();
    }

    pub fn start<F: Fn(I) -> O>(&mut self) -> F {
        for r in 0..self.runs {
            // set seed here

            self.generate_initial_population();

            for g in 0..self.generations {
                self.generate_next_population();
            }
        }

        todo!()
    }
}
