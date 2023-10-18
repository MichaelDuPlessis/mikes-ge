pub mod ge;
pub mod grammar;

use grammar::Grammar;

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    X,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Grammar for Expr {
    type Input = f64;

    type Output = f64;

    fn run(&self, input: &Self::Input) -> Self::Output {
        match self {
            Expr::Add(expr1, expr2) => expr1.run(input) + expr2.run(input),
            Expr::Mul(expr1, expr2) => expr1.run(input) * expr2.run(input),
            Expr::X => *input,
            Expr::One => 1.0,
            Expr::Two => 2.0,
            Expr::Three => 3.0,
            Expr::Four => 4.0,
            Expr::Five => 5.0,
        }
    }

    fn generate(chromosome: &[u8]) -> Self {
        Self::generate_helper(&mut 0, chromosome.as_ref())
    }
}

impl Expr {
    fn generate_helper(pos: &mut usize, chromosome: &[u8]) -> Self {
        // dbg!(chromosome);
        let p = *pos % chromosome.len();
        if *pos / chromosome.len() > 3 {
            let terminal = match chromosome[p] % 6 {
                0 => Self::X,
                1 => Self::One,
                2 => Self::Two,
                3 => Self::Three,
                4 => Self::Four,
                5 => Self::Five,
                _ => panic!("Cannot get here"),
            };

            *pos += 1;
            return terminal;
        }

        *pos += 1;
        match chromosome[p] % 8 {
            0 => Self::Add(
                Box::new(Self::generate_helper(pos, chromosome)),
                Box::new(Self::generate_helper(pos, chromosome)),
            ),
            1 => Self::Mul(
                Box::new(Self::generate_helper(pos, chromosome)),
                Box::new(Self::generate_helper(pos, chromosome)),
            ),
            2 => Self::X,
            3 => Self::One,
            4 => Self::Two,
            5 => Self::Three,
            6 => Self::Four,
            7 => Self::Five,
            _ => panic!("Cannot get here"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ge::GE;

    use super::*;

    #[test]
    fn test() {
        let train = (-10..10)
            .map(|x| {
                let x = x as f64;
                (x, x * x + 2.0)
            })
            .collect::<Vec<_>>();
        let mut ge: GE<f64, f64, Expr> = GE::new(100, (0.5, 0.5, 0.0), 2, 10, 1000, 4, 1, &train);
        let expr = Expr::generate(&ge.start());
        dbg!(expr.run(&1.0));
    }
}
