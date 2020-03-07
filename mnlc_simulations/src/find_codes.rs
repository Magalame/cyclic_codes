use std::cmp::{max,min};
use believer::{ParityCheckMatrix, EDBuilder,ClassicalSimulator,DecoderBuilder,Simulator,add_checks_mut};
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

pub fn test_family(n0: usize, k0: usize, iter: usize){
    
    let lim_per_g = 5;

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_dist_gen_char_n0{}_k0{}",n0,k0)).ok().unwrap();
    //let m_wtr = Arc::new(Mutex::new(wtr));

    for w in &[5] {


        for i in 1..iter+1 {

            let g_count: HashMap<String, usize> = HashMap::new();
            let m_g_count = Arc::new(Mutex::new(g_count));

            let mut poly_index: Vec<usize> = Vec::with_capacity(*w); 
            
            let n = n0*i;
            let target_k = k0*i;

            init(&mut poly_index, *w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            let n_workers = 8;
            let pool = Pool::<ThunkWorker<Option<SimulRes>>>::new(n_workers);

            let (tx, rx) = channel();

            for code in poly {
                let a_g_count = Arc::clone(&m_g_count);
                pool.execute_to(tx.clone(), Thunk::of(move ||{
                    let x = conditional_simul(a_g_count, 5, &code, n, target_k);
                    x
                }));
            }

            

            for res in rx {
                if let Some(sres) = res {
                    wtr.write_record(&[format!("{}",n),format!("{}",target_k), format!("{}",sres.err.unwrap()),format!("{}",sres.dist.unwrap()),format!("{}",sres.g.unwrap())]).ok();
                    wtr.flush().ok();
                }
                if pool.active_count() == 0 && pool.queued_count() == 0 {
                    break;
                }
            }


        }

    }

    
    

}

pub fn test_family_range(n0: usize, k0: usize, iter: usize){
    
    let lim_per_g = 5;
    let target_ratio = (k0 as f64)/(n0 as f64);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_wmul_n0{}_k0{}",n0,k0)).ok().unwrap();
    //let m_wtr = Arc::new(Mutex::new(wtr));

    for w in &[5] {


        for i in n0..(n0+iter+1) {

            let g_count: HashMap<String, usize> = HashMap::new();
            let m_g_count = Arc::new(Mutex::new(g_count));

            let mut poly_index: Vec<usize> = Vec::with_capacity(*w); 
            
            let n = i;

            init(&mut poly_index, *w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            let n_workers = 8;
            let pool = Pool::<ThunkWorker<Option<SimulRes>>>::new(n_workers);

            let (tx, rx) = channel();

            for code in poly {
                let a_g_count = Arc::clone(&m_g_count);
                pool.execute_to(tx.clone(), Thunk::of(move ||{
                    let x = conditional_simul_range(a_g_count, lim_per_g, &code, n, target_ratio);
                    x
                }));
            }

            

            for res in rx {
                if let Some(sres) = res {
                    wtr.write_record(&[format!("{}",n),format!("{}",sres.k), format!("{}",sres.err.unwrap()),format!("{}",sres.dist.unwrap()),format!("{}",sres.g.unwrap())]).ok();
                    wtr.flush().ok();
                }
                if pool.active_count() == 0 && pool.queued_count() == 0 {
                    break;
                }
            }


        }

    }

    
    

}

pub fn find_max_k_5(n: usize) -> usize{

    let mut max_max = 0;

    if let Some(max_found) = (1..n-3).into_par_iter().map( 
        |i_0| {
            let mut max_k = 0;
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);

                        let k = n-matrix.rank();

                        if k>max_k {
                            max_k = k;
                        }

                    }
                }
            }
            max_k
        }

    ).max() {
        max_max = max_found;
    }

    max_max

}

pub fn gen_to_pcm(gen: String) -> ParityCheckMatrix{
    //let len: usize = gen.split("  ").nth(0).unwrap().split(" ").nth(0).unwrap().parse().unwrap();
    let poly_str = gen.split("  ").nth(1).unwrap().split(" ");
    let mut w = 0;
    for i in poly_str.clone() {
        if i == "1" {
            w += 1;
        }
    }
    let mut poly: Vec<usize> = Vec::with_capacity(w);
    let mut i = 0;
    for check in poly_str {
        if check == "1" {
            poly.push(i);
        }
        i +=1 ;
    }

    ParityCheckMatrix::circulant_right_better(&poly, i+1)
}

pub fn min_distance(code: &Vec<usize>, n: usize) -> usize{

    let gen = gen_str(code, n);

    let matrix = gen_to_pcm(gen);

    let w = code.len();
    let mut sum: Vec<usize> = Vec::with_capacity(2*w);

    matrix.checks_iter().map( |row1| {
        let min1 = matrix.checks_iter().filter_map( |row2| {
            add_checks_mut(row1.positions(), row2.positions(), &mut sum);
            let w = sum.len();
            if w == 0 { //for the case where we add the row to itself
                None
            } else {
                Some(w)
            }
        }).min().unwrap();

        let min2 = row1.positions().len();

        min(min1, min2)

    }).min().unwrap()

}


pub fn conditional_simul(ag_count: Arc<Mutex<HashMap<String,usize>>>, g_lim:usize, code: &Vec<usize>, n: usize, target_k: usize) -> Option<SimulRes>{

    let matrix = ParityCheckMatrix::circulant_right_better(code, n);
    let code_k = n-matrix.rank();

    if target_k == code_k {

        let g = char_str(code,n);

        let mut g_count = ag_count.lock().unwrap();

        
        if !g_count.contains_key(&g) {

            g_count.insert(g,1);

            drop(g_count);

            return simulate_code(code, n, target_k)

        } else {
            if let Some(count) = g_count.get_mut(&g) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return simulate_code(code, n, target_k)
                } else {

                    drop(count);
                    drop(g_count);

                    return None
                }
            
            }

            return None
        }

    } else {
        None
    }



}

pub fn conditional_simul_range(ag_count: Arc<Mutex<HashMap<String,usize>>>, g_lim:usize, code: &Vec<usize>, n: usize, target_ratio: f64) -> Option<SimulRes>{

    let code_k = code_k(code, n);

    let code_ratio = (code_k as f64)/(n as f64);

    if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

        let g = char_str(code,n);

        let mut g_count = ag_count.lock().unwrap();

        
        if !g_count.contains_key(&g) {

            g_count.insert(g,1);

            drop(g_count);

            return simulate_code(code, n, code_k)

        } else {
            if let Some(count) = g_count.get_mut(&g) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return simulate_code(code, n, code_k)
                } else {

                    drop(count);
                    drop(g_count);

                    return None
                }
            
            }

            return None
        }

    } else {
        None
    }



}

pub fn simulate_code(code: &Vec<usize>, n: usize, target_k: usize) -> Option<SimulRes> {

    let decoder_builder = EDBuilder::new(0.49);
    let err = get_err(code, n, &decoder_builder);
    let g = char_str(code, n);
    //println!("enter");
    let dist = min_distance(code,n);
    println!("{:?}", code);
    //println!("out");
    //let cr = code_cr(code, n);
    
    //let mul = multiplicity(&code, n);


    return Some(SimulRes {
        n,
        k:target_k,
        err: Some(err),
        cr: None,
        dist: Some(dist),
        g: Some(g),
        f: None,
        mul: None,
        coef: None
    })

}

pub struct SimulRes {
    n: usize,
    k: usize,
    err: Option<f64>, //err rate
    cr: Option<usize>, // cr number
    dist: Option<usize>,
    g: Option<String>,
    f: Option<String>,
    mul: Option<f64>,
    coef: Option<Vec<usize>>
}

pub fn get_err(code: &Vec<usize>, n: usize, decoder_builder: &EDBuilder) -> f64 {

    let matrix = ParityCheckMatrix::circulant_right_better(code, n);

    let decoder = decoder_builder.from_code(matrix);
    let simulator = ClassicalSimulator::new(decoder);
    let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

    err_rate

}

pub fn code_cr(code: &Vec<usize>, n:usize) -> usize {
    let (v1,v2) = poly_to_edgelist(code,n);
    cr(&v1,&v2)
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
        let indexes = self.indexes.as_mut().unwrap();
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

            Some(indexes.clone())
        } else {
            None
        }

        
    }
}