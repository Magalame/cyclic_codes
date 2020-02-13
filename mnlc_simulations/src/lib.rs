#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
use believer::{Decoder,DecoderBuilder, Code};
use believer::{Simulator};
use rayon::prelude::*;
//use rayon::iter::{Map,ParallelIterator};
//extern crate libc;



pub trait CodeGenerator<C,Cks,D> : Sync where 
C: Code<Cks> + Send + Sync,
D: Decoder {

    type SimulatorType: Simulator<D>;

    fn generate(&self) -> C;

    fn simulate<B: Sync>(
        &self,
        decoder_builder: B,
        n_codes: usize,
        n_trials: usize,
    ) -> Vec<(C,f64)> where

    B: DecoderBuilder<Cks,D>,
    
    {

        let mut codes_failures: Vec<(C,f64)> = Vec::with_capacity(n_codes);

        (0..n_codes).into_par_iter().map( |_| { 
            let code = self.generate();
            let checks = code.get_checks();
            let decoder = decoder_builder.from_code(checks);
            let simulator = Self::SimulatorType::new(decoder);
            let result = simulator.simulate_n_trials(n_trials).failure_rate();

            (code, result)
        }
        ).collect_into_vec(&mut codes_failures);

        codes_failures
    }

}

// pub mod increasing_range_codes;
// pub use increasing_range_codes::*;

// pub mod random_codes;
// pub use random_codes::*;

pub mod generalized_bicycle_codes;
pub use generalized_bicycle_codes::*;

pub mod find_codes;
pub use find_codes::*;

pub mod qc;
pub use qc::*;

pub mod graphs;
pub use graphs::*;

pub mod flint;
pub use flint::*;