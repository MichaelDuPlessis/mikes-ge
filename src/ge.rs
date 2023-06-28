use crate::grammer::{Distance, Grammer};
use rand::{self, Rng};

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
        train: Vec<(I, O)>,
        grammer: G,
        runs: usize,
    ) -> Self {
        assert!(min_len < max_len);
        assert!(weights.0 + weights.1 + weights.2 == 1.0);

        Self {
            size,
            weights,
            min_len,
            max_len,
            generations,
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

    fn raw_fitness(&self, chromosome: &Chromosome) {
        let individual = G::generate(&chromosome);
        let res: f64 = self
            .train
            .iter()
            .map(|(input, expected)| (expected.distance(&individual.run(input))).abs())
            .sum();
    }

    fn generate_next_population(&mut self) {}

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
