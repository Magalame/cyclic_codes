use super::CodeGenerator;
use believer::ParityCheckMatrix;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};

pub struct IncreasingRangeCodeGenerator {
    n_bits: usize,
    n_checks: usize,
    max_bit_degree: usize,
    max_check_degree: usize,
    initial_distance_rate: f64,
    minimal_cycle_length: usize,
}

// impl CodeGenerator for IncreasingRangeCodeGenerator {
//     fn generate(&self) -> ParityCheckMatrix {
//         // println!("Generating code ...");
//         let mut checks = Vec::with_capacity(self.n_checks);
//         let mut bit_distribution = BitDistribution::new(
//             self.n_bits,
//             self.max_bit_degree,
//             self.initial_distance_rate,
//             self.minimal_cycle_length,
//         );

//         let mut last_update = 0;
//         for index in 0..self.n_checks {
//             if let Some(check) = self.random_check(&mut bit_distribution) {
//                 checks.push(check);
//             }
//             if self.n_checks - index < (self.n_checks - last_update) / 2 {
//                 bit_distribution.update_distance_rate();
//                 last_update = index;
//             }
//         }
//         ParityCheckMatrix::new(checks, self.n_bits)
//     }
// }

impl IncreasingRangeCodeGenerator {
    pub fn new(
        n_bits: usize,
        n_checks: usize,
        max_bit_degree: usize,
        max_check_degree: usize,
        initial_distance_rate: f64,
        minimal_cycle_length: usize,
    ) -> Self {
        Self {
            n_bits,
            n_checks,
            max_bit_degree,
            max_check_degree,
            initial_distance_rate,
            minimal_cycle_length,
        }
    }

    fn random_check(&self, distribution: &mut BitDistribution) -> Option<Vec<usize>> {
        let mut rng = thread_rng();
        let deg = rng.gen_range(2, self.max_check_degree + 1);
        let mut check = Vec::with_capacity(deg);
        for _ in 0..deg {
            distribution.add_bit_to_check(&mut check);
        }
        if check.len() < 2 {
            None
        } else {
            Some(check)
        }
    }
}

struct BitDistribution {
    availabilities: Vec<usize>,
    adjacencies: Vec<Vec<usize>>,
    adjacency_depth: usize,
    distance_rate: f64,
}

impl BitDistribution {
    fn add_bit_to_check(&mut self, check: &mut Vec<usize>) {
        let availables = self.availables(check);
        if let Some(bit) = check.first().map_or_else(
            || Some(availables[thread_rng().gen_range(0, availables.len())]),
            |first| {
                self.random_bit(&availables, first).map(|bit| {
                    self.update_adjacencies(bit, check);
                    bit
                })
            },
        ) {
            check.push(bit);
        }
    }

    fn availables(&self, check: &[usize]) -> Vec<usize> {
        let x = self
            .availabilities
            .iter()
            .enumerate()
            .filter_map(|(bit, avaibility)| {
                if self.bit_is_not_adjacent_to_check(bit, check) && *avaibility > 0 {
                    Some(bit)
                } else {
                    None
                }
            })
            .collect();
        // println!("Avail: {:?}", x);
        x
    }

    fn bit_is_not_adjacent_to_check(&self, bit: usize, check: &[usize]) -> bool {
        // println!("------------");
        // println!("CHECK: {:?}", check);
        // println!("BIT: {}", bit);
        check.iter().all(|b| {
            let x = self.bit_adjacencies(b, self.adjacency_depth);
            // println!("Bit: {}", b);
            // println!("Adj: {:?}", x);
            !x.contains(&bit)
        })
    }

    fn bit_adjacencies(&self, bit: &usize, depth: usize) -> Vec<usize> {
        let mut adj = if depth == 0 {
            Vec::with_capacity(1)
        } else if depth == 1 {
            self.adjacencies[*bit].clone()
        } else {
            self.adjacencies[*bit]
                .iter()
                .flat_map(|b| {
                    let mut adjacent_bits = self.bit_adjacencies(b, depth - 1);
                    adjacent_bits.push(*b);
                    adjacent_bits
                })
                .collect()
        };
        adj.push(*bit);
        adj
    }

    fn new(
        n_bits: usize,
        max_bit_degree: usize,
        distance_rate: f64,
        minimal_cycle_length: usize,
    ) -> Self {
        Self {
            availabilities: vec![max_bit_degree; n_bits],
            adjacencies: vec![Vec::new(); n_bits],
            adjacency_depth: if minimal_cycle_length < 6 {
                0
            } else {
                minimal_cycle_length / 2 - 2
            },
            distance_rate,
        }
    }

    fn random_bit(&self, availables: &[usize], source: &usize) -> Option<usize> {
        let probs = self.get_probs(availables, source);
        let random_number: f64 = thread_rng().gen();
        let mut sum = 0.0;
        probs.iter().zip(availables.iter()).find_map(|(prob, bit)| {
            sum += prob;
            if random_number <= sum {
                Some(*bit)
            } else {
                None
            }
        })
    }

    fn get_probs(&self, availables: &[usize], source: &usize) -> Vec<f64> {
        let mut probs: Vec<f64> = availables
            .iter()
            .map(|bit| (-self.distance_rate * self.distance(bit, source)).exp())
            .collect();
        let sum: f64 = probs.iter().sum();
        probs.iter_mut().for_each(|prob| *prob /= sum);
        probs
    }

    fn update_distance_rate(&mut self) {
        self.distance_rate /= 2.0;
    }

    fn distance(&self, bit_0: &usize, bit_1: &usize) -> f64 {
        let d = max(bit_0, bit_1) - min(bit_0, bit_1);
        min(d, self.availabilities.len() - d) as f64
    }

    fn update_adjacencies(&mut self, bit: usize, check: &[usize]) {
        self.adjacencies[bit].append(&mut check.clone().to_vec());
        check
            .iter()
            .for_each(|b| self.adjacencies[*b].push(bit.clone()))
    }
}
