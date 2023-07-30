pub trait Grammer {
    type Input;
    // this means that the output must have some method that returns a float this is so that we are
    // able to determine how far it is from the goal
    type Output: Distance;

    fn run(&self, input: &Self::Input) -> Self::Output;

    fn generate(chromosome: &[u8]) -> Self;
}

// use when calculating fitness
pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

// this sucks so much I want specialization
impl Distance for usize {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for u128 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for u64 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for u32 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for u16 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for u8 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for i128 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for isize {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for i64 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for i32 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for i16 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for i8 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}

impl Distance for f64 {
    fn distance(&self, other: &Self) -> f64 {
        self - other
    }
}

impl Distance for f32 {
    fn distance(&self, other: &Self) -> f64 {
        (self - other) as f64
    }
}
