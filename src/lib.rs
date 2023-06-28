use crate::ge::GE;
use grammer::Grammer;

mod ge;
mod grammer;

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    X,
    Y,
}

impl Grammer for Expr {
    type Input = (usize, usize);
    type Output = usize;

    fn run(self, input: Self::Input) -> Self::Output {
        match self {
            Expr::Add(expr1, expr2) => expr1.run(input) + expr2.run(input),
            Expr::Sub(expr1, expr2) => expr1.run(input) - expr2.run(input),
            Expr::X => input.0,
            Expr::Y => input.1,
        }
    }

    fn generate(chromosome: &Vec<u8>) -> Self {
        Self::generate_helper(&mut 0, chromosome)
    }
}

impl Expr {
    fn generate_helper(pos: &mut usize, chromosome: &Vec<u8>) -> Self {
        let p = *pos % chromosome.len();
        if *pos / chromosome.len() > 3 {
            let terminal = if chromosome[p] % 2 == 0 {
                Self::X
            } else {
                Self::Y
            };

            *pos += 1;
            return terminal;
        }

        *pos += 1;
        match chromosome[p] % 4 {
            0 => Self::Add(
                Box::new(Self::generate_helper(pos, chromosome)),
                Box::new(Self::generate_helper(pos, chromosome)),
            ),
            1 => Self::Sub(
                Box::new(Self::generate_helper(pos, chromosome)),
                Box::new(Self::generate_helper(pos, chromosome)),
            ),
            2 => Self::X,
            3 => Self::Y,
            _ => panic!("Cannot get here"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expr = Expr::Add(Box::new(Expr::X), Box::new(Expr::Y));
        assert_eq!(5, expr.run((2, 3)));
    }
}
