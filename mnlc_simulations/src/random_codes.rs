use super::CodeGenerator;
use believer::ParityCheckMatrix;
use rand::{thread_rng, Rng};

pub struct RandomCodeGenerator {
    n_bits: usize,
    n_checks: usize,
    max_bit_degree: usize,
    max_check_degree: usize,
}

// impl CodeGenerator for RandomCodeGenerator {
//     fn generate(&self) -> ParityCheckMatrix {
//         let mut availabilities = BitAvailabilities::new(self.n_bits, self.max_bit_degree);
//         let mut checks = Vec::with_capacity(self.n_checks);
//         for _ in 0..self.n_checks {
//             checks.push(self.random_check(&mut availabilities));
//         }
//         ParityCheckMatrix::new(checks, self.n_bits)
//     }
// }

impl RandomCodeGenerator {
    pub fn new(
        n_bits: usize,
        n_checks: usize,
        max_bit_degree: usize,
        max_check_degree: usize,
    ) -> Self {
        Self {
            n_bits,
            n_checks,
            max_bit_degree,
            max_check_degree,
        }
    }

    fn random_check(&self, availabilities: &mut BitAvailabilities) -> Vec<usize> {
        let mut rng = thread_rng();
        let deg = rng.gen_range(2, self.max_check_degree + 1);
        let mut check = Vec::with_capacity(deg);
        for _ in 0..deg {
            availabilities.add_bit_to_check(&mut check);
        }
        check
    }
}


struct BitAvailabilities {
    availabilities: Vec<usize>,
    total: usize,
}

impl BitAvailabilities {
    fn new(n_bits: usize, max_bits_degree: usize) -> Self {
        Self {
            availabilities: vec![max_bits_degree; n_bits],
            total: n_bits * max_bits_degree,
        }
    }

    fn availables(&self, check: &[usize]) -> usize {
        let unavaible: usize = self
            .availabilities
            .iter()
            .enumerate()
            .filter(|(bit, _)| check.iter().any(|b| b == bit))
            .map(|(_, availability)| availability)
            .sum();
        self.total - unavaible
    }

    fn add_bit_to_check(&mut self, check: &mut Vec<usize>) {
        let random_number = thread_rng().gen_range(0, self.availables(check));
        let mut sum = 0;
        let random_bit = self
            .availabilities
            .iter()
            .enumerate()
            .find_map(|(bit, availability)| {
                if check.iter().all(|b| *b != bit) {
                    sum += availability;
                    if random_number <= sum {
                        Some(bit)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();
        check.push(random_bit);
        self.availabilities[random_bit] -= 1;
        self.total -= 1;
    }
}
