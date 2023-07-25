use crate::grammer::{Distance, Grammer};
use rand::{self, seq::SliceRandom, Rng};
use std::{collections::HashMap, marker::PhantomData};

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
    population: Vec<Chromosome>,
    _grammer: PhantomData<G>,
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
            population: Vec::with_capacity(size),
            _grammer: PhantomData,
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
        let individual = G::generate(chromosome);
        self.train
            .iter()
            .map(|(input, expected)| (expected.distance(&individual.run(input))).abs())
            .sum()
    }

    // performs tournament selection on the population and returns the index of the chosen individual
    fn tournament_selection<'a>(
        &'a self,
        cache: &mut HashMap<&'a Chromosome, f64>,
    ) -> &'a Chromosome {
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
        let mut cache = HashMap::new();
        let mut new_population = Vec::with_capacity(self.size);

        for _ in 0..(self.size as f32 * self.weights.0) as usize {
            let (c1, c2) = self.crossover(
                self.tournament_selection(&mut cache),
                self.tournament_selection(&mut cache),
            );

            new_population.push(c1);
            new_population.push(c2);
        }

        for _ in 0..(self.size as f32 * self.weights.1) as usize {
            let c = self.mutation(self.tournament_selection(&mut cache));

            new_population.push(c);
        }

        for _ in 0..(self.size as f32 * self.weights.2) as usize {
            let c = self.tournament_selection(&mut cache).clone();

            new_population.push(c);
        }

        while new_population.len() < self.size {
            let c = self.tournament_selection(&mut cache).clone();

            new_population.push(c);
        }

        self.population = new_population;
    }

    // ====================================================================
    // genetic operators
    fn crossover(
        &self,
        chromosome1: &Chromosome,
        chromosome2: &Chromosome,
    ) -> (Chromosome, Chromosome) {
        let (chromosome1, chromosome2) = if chromosome2.len() < chromosome1.len() {
            (chromosome2, chromosome1)
        } else {
            (chromosome1, chromosome2)
        };

        let point1: usize = rand::random::<usize>() % chromosome1.len();
        let point2: usize = rand::random::<usize>() % chromosome2.len();
        let (point1, point2) = (point1.min(point2), point1.max(point2));

        let c1_len = point1 + (point2 - point1) + (chromosome1.len() - point2);
        let c2_len = point1 + (point2 - point1) + (chromosome2.len() - point2);

        let mut new_c1 = Vec::with_capacity(c1_len);
        let mut new_c2 = Vec::with_capacity(c2_len);

        // create first new chromosome
        new_c1.extend_from_slice(&chromosome1[..point1]);
        new_c1.extend_from_slice(&chromosome2[point1..point2]);
        new_c1.extend_from_slice(&chromosome1[point2..]);

        // create second new chromosome
        new_c2.extend_from_slice(&chromosome2[..point1]);
        new_c1.extend_from_slice(&chromosome1[point1..point2]);
        new_c1.extend_from_slice(&chromosome2[point2..]);

        (new_c1, new_c2)
    }

    fn mutation(&self, chromosome: &Chromosome) -> Chromosome {
        let mut chromosome = chromosome.clone();

        *chromosome.choose_mut(&mut rand::thread_rng()).unwrap() = rand::random();
        chromosome
    }

    pub fn start(&mut self) -> Chromosome {
        for r in 0..self.runs {
            // set seed here

            self.generate_initial_population();

            for g in 0..self.generations {
                self.generate_next_population();
            }
        }

        let best = self
            .population
            .iter()
            .max_by(|x, y| self.raw_fitness(x).total_cmp(&self.raw_fitness(y)))
            .unwrap();

        best.clone()
    }
}
