use std::cmp::{max,min};
use believer::{ParityCheckMatrix, EDBuilder,ClassicalSimulator,DecoderBuilder,Simulator};
use rayon::prelude::*;
use crate::graphs::*;
use crate::qc::*;
use crate::flint::*;
use std::ops::Range;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use gcd::Gcd;
use std::collections::HashSet;
use std::collections::HashMap;

use std::sync::mpsc::channel;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};

pub fn init(vec: &mut Vec<usize>, w:usize){

    vec.clear();

    for i in 0..w {
        vec.push(i);
    }
}

pub fn test_family_5(n0: usize, k0: usize, iter: usize){
    
    let w = 5;
    let lim_per_g = 5;
    let mut results: Vec<(usize, usize, f64, String)>  = Vec::with_capacity(iter*5*lim_per_g);// n, k, err_rate, g // it's reasonable to assume no more than 5 diff g on average for each n

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_test_fcwpf_n0{}_k0{}",n0,k0)).ok().unwrap();
    let m_wtr = Arc::new(Mutex::new(wtr));

    let g_count = HashMap::new();
    let m_g_count = Arc::new(Mutex::new(g_count));

    let decoder_builder = EDBuilder::new(0.49);

    let mut poly_index: Vec<usize> = Vec::with_capacity(w);
    init(&mut poly_index, w);

    for i in 1..iter+1 {
        let n = n0*i;
        let target_k = k0_i;

        let poly = Poly5{
            indexes: Some(poly_index),
            n
        };
    }

    let n_workers = 8;
    let pool = Pool::<ThunkWorker<i32>>::new(n_workers);

    let (tx, rx) = channel();
    for code in poly {
        pool.execute_to(tx.clone(), Thunk::of(move || i * i));
    }

}

pub fn simulate_code_5(code: (usize, usize, usize, usize, usize)){

}



struct Poly5 {
    indexes: Option<Vec<usize>>,
    n: usize
}

impl Iterator for Poly5 {
    type Item = (usize, usize, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let w = 5;
        let n = self.n;
        let indexes_opt = self.indexes.as_mut();

        match indexes_opt{
            Some(indexes) => if indexes[1] <= n - w + 1 {
                for i in 2..w {
                    if indexes[i] == n - w + i {
                        indexes[i-1] = indexes[i-1] + 1;
                        indexes[i] = indexes[i-1] + 1;

                        for j in (i + 1)..w {
                            indexes[j] = indexes[i] + j - i;
                        }

                        break;
                    } else if i == w-1 {
                        indexes[i] = indexes[i] + 1;
                    }
                }

                Some((indexes[0],indexes[1],indexes[2],indexes[3],indexes[4]))
            } else {
                None
            }

            _ => None
        }
    }
}

struct Poly {
    indexes: Option<Vec<usize>>,
    n: usize
}

impl Iterator for Poly {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let mut indexes = self.indexes.as_mut().unwrap().clone();
        let w = indexes.len();

        
        if indexes[1] <= n - w + 1 {
            for i in 2..w {
                if indexes[i] == n - w + i {
                    indexes[i-1] = indexes[i-1] + 1;
                    indexes[i] = indexes[i-1] + 1;

                    for j in (i + 1)..w {
                        indexes[j] = indexes[i] + j - i;
                    }

                    break;
                } else if i == w-1 {
                    indexes[i] = indexes[i] + 1;
                }
            }

            Some(indexes)
        } else {
            None
        }

        
    }
}