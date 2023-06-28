pub trait Grammer {
    type Input;
    // this means that the output must have some method that returns a float this is so that we are
    // able to determine how far it is from the goal
    type Output: Distance;

    fn run(&self, input: &Self::Input) -> Self::Output;

    fn generate(chromosome: &Vec<u8>) -> Self;
}

// use when calculating fitness
pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

// implementation for standard library types
impl Distance for f64 {
    fn distance(&self, other: &Self) -> f64 {
        self - other
    }
}

impl Distance for usize {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}
