pub trait Grammer {
    type Input;
    // this specifies that the output has to be of some kind of float
    //
    // This should change later so that Output must instead implement some custom type that
    // specifies a method that returns a float this is because Output need not necessarily be a
    // float
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
