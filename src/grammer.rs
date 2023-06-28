pub trait Grammer {
    type Input;
    type Output;

    fn run(self, input: Self::Input) -> Self::Output;

    fn generate(chromosome: &Vec<u8>) -> Self;
}
