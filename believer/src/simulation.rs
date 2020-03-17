//! Toolbox to simulate error correction codes performance.

use crate::{Decoder, DecodingResult};
use rayon::prelude::*;


pub trait Simulator<D>
where D: Decoder
{ 

    fn new(decoder: D) -> Self;

    fn decoder(&self) -> &D;

    fn simulate_n_iterations(&self, n_iterations: usize) -> SimulationResult {
        let decoder = self.decoder();
        let successes: usize = (0..n_iterations)
            .map(|_| {
                let error = decoder.random_error();
                decoder.decode(&error)
            })
            .filter(|decoding| decoding.succeed())
            .count();

        SimulationResult {
            successes,
            failures: n_iterations - successes,
        }
    }

    fn simulate_until_failures_are_found_serial_mut(
        &self,
        n_failures: usize,
    ) -> SimulationResult {
        let decoder = self.decoder();
        let mut work_pcm = decoder.copy_pcm();
        let mut error: Vec<usize> = Vec::with_capacity(work_pcm.n_bits());
        let successes = (0..n_failures).map(|_| {
                        let mut successes = 0;
                        let mut has_failed = false;
                        while !has_failed && successes < 1000000{
                            decoder.random_error_mut(&mut error);
                            if decoder.decode_mut(&error, &mut work_pcm).succeed() {
                                successes += 1;
                            } else {
                                has_failed = true;
                            }
                        }
                        successes
            })
            .sum();
         //println!("nb of successes:{}", successes);
         SimulationResult {
            successes,
            failures: n_failures,
        }
    }

    fn simulate_until_failures_are_found_serial(
        &self,
        n_failures: usize,
    ) -> SimulationResult {
        let decoder = self.decoder();
        let successes = (0..n_failures).map(|_| {
                        let mut successes = 0;
                        let mut has_failed = false;
                        while !has_failed && successes < 1000000 {
                            let error = decoder.random_error();
                            if decoder.decode(&error).succeed() {
                                successes += 1;
                            } else {
                                has_failed = true;
                            }
                        }
                        successes
            })
            .sum();

         SimulationResult {
            successes,
            failures: n_failures,
        }
    }

    fn simulate_until_failures_are_found(
        &self,
        n_threads: usize,
        n_failures_per_thread: usize,
    ) -> SimulationResult {
        let decoder = self.decoder();
        let successes = (0..n_threads).into_par_iter()
            .map::<_, usize>(|_| {
                (0..n_failures_per_thread)
                    .map(|_| {
                        let mut successes = 0;
                        let mut has_failed = false;
                        while !has_failed && successes < 1000000{
                            let error = decoder.random_error();
                            if decoder.decode(&error).succeed() {
                                successes += 1;
                            } else {
                                has_failed = true;
                            }
                        }
                        successes
                    })
                    .sum()
            })
            .sum();

         SimulationResult {
            successes,
            failures: n_failures_per_thread * n_threads,
        }
    }

    fn simulate_n_trials(
        &self,
        n_trials: usize
    ) -> SimulationResult {
        let decoder = self.decoder();
        
        let mut successes = 0;
        let mut failures = 0;

        for _ in 0..n_trials {
            let error = decoder.random_error();
            if decoder.decode(&error).succeed() {
                successes += 1;
            } else {
                failures += 1;
            }
        }

         SimulationResult {
            successes,
            failures: failures,
        }
    }

}

pub struct CSSSimulator<D>
where D: Decoder
{
    decoder: D
}

impl< D> Simulator< D> for CSSSimulator< D>
where D: Decoder {

    fn new(decoder: D) -> Self {
        Self { decoder }
    }

    fn decoder(&self) -> &D{
        &self.decoder
    }

}

pub struct ClassicalSimulator< D>
where D: Decoder
{
    decoder:  D
}

impl< D> Simulator< D> for ClassicalSimulator< D>
where D: Decoder
{

    fn new(decoder: D) -> Self {
        Self { decoder }
    }

    fn decoder(&self) -> &D{
        &self.decoder
    }

}


pub struct SimulationResult {
    successes: usize,
    failures: usize,
}

impl SimulationResult {
    pub fn failure_rate(&self) -> f64 {
        self.failures() as f64 / self.total() as f64
    }

    pub fn failures(&self) -> usize {
        self.failures
    }

    pub fn success_rate(&self) -> f64 {
        self.successes() as f64 / self.total() as f64
    }

    pub fn successes(&self) -> usize {
        self.successes
    }

    pub fn total(&self) -> usize {
        self.failures() + self.successes()
    }
}

pub struct CSSSimulationResult {
    successes: usize,
    x_failures: usize,
    z_failures: usize,
    x_and_z_failures: usize,
}

impl CSSSimulationResult {

    pub fn x_failures(&self) -> usize {
        self.x_failures
    }

    pub fn x_failure_rate(&self) -> f64 {
        self.x_failures() as f64 / self.total() as f64
    }

    pub fn z_failures(&self) -> usize {
        self.z_failures
    }

    pub fn z_failure_rate(&self) -> f64 {
        self.z_failures() as f64 / self.total() as f64
    }

    pub fn x_and_z_failures(&self) -> usize {
        self.x_and_z_failures
    }

    pub fn x_and_z_failure_rate(&self) -> f64 {
        self.x_and_z_failures() as f64 / self.total() as f64
    }

    pub fn total_failures(&self) -> usize {
        self.x_failures + self.z_failures + self.x_and_z_failures
    }

    pub fn total_failure_rate(&self) -> f64 {
        self.total_failures() as f64 / self.total() as f64
    }

    pub fn successes(&self) -> usize {
        self.successes
    }

    pub fn total(&self) -> usize {
        self.total_failures() + self.successes()
    }

}