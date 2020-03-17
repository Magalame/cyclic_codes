use std::cmp::{max,min};
use believer::{ParityCheckMatrix,QuantumErasureDecoder, Decoder};
use rayon::prelude::*;
use crate::graphs::*;
use crate::qc::*;
use crate::flint::*;
use std::ops::Range;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::str::FromStr;

// use gcd::Gcd;
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

fn str_to_vec(s: &str) -> Vec<usize> {
    s.trim_matches(|p| p == '[' || p == ']')
                                .split(',').map(|s| s.trim()) 
                                .map(|x| usize::from_str(x).unwrap())
                                .collect()
}
pub fn landscape_from_classical(n0: usize, iter: usize){
    
    let p_physical = 0.2;

    for n in n0..n0 + iter {

        println!("at n:{}",n);

        let wtr = Arc::new(Mutex::new(csv::Writer::from_path(&format!("/home/nouey/Projects/mnlc2/mnlc_simulations/data/landscape_from_classical_n0:{}_p:{}_r:0.1",n,p_physical)).ok().unwrap()));

        let rdr_res = csv::ReaderBuilder::new().has_headers(false)
                                          .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n));

        if let Ok(rdr) = rdr_res {

            let mut rdr = rdr;

            for row in rdr.records() {
                let row = row.unwrap();
                let mut rdr2 = csv::ReaderBuilder::new().has_headers(false)
                                                   .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n)).ok().unwrap();

                for row2 in rdr2.records() {

                    let row2 = row2.unwrap();
                    let code_a = str_to_vec(&row[4]);
                    let k_a: usize = row[1].parse().unwrap();
                    let g_a: String = row[3].parse().unwrap();
                    let code_b = str_to_vec(&row2[4]);
                    let k_b: usize = row2[1].parse().unwrap();
                    let g_b: String = row2[3].parse().unwrap();

                    let x_opt = simulate_code_quantum_classical(&code_a, k_a, g_a, &code_b, k_b, g_b, n, p_physical);

                    if let Some(x) = x_opt {
                        let mut wtr = wtr.lock().unwrap();
                        wtr.write_record(&[format!("{}",x.n),
                                            format!("{}",x.k),
                                            format!("{}",x.k_a),
                                            format!("{}",x.k_b),   
                                            format!("{}",x.err.unwrap()),
                                            format!("{}",x.g_a.unwrap()), 
                                            format!("{}",x.g_b.unwrap()),row[4].to_string() , row2[4].to_string()]).ok();
                            wtr.flush().ok();
                    } else {}
                }
            }

        } else {}
    }

}

pub fn landscape_from_classical_fixed_fam(n0: usize, iter: usize){
    let p_physical = 0.2;

    let n_workers = 8;
    let pool = Pool::<ThunkWorker<()>>::new(n_workers);

    let (tx, rx) = channel();


    for n in n0..n0 + iter {

        println!("at n:{}",n);

        let wtr = Arc::new(Mutex::new(csv::Writer::from_path(&format!("/home/nouey/Projects/mnlc2/mnlc_simulations/data/landscape_from_classical_fixed_n0:{}_p:{}_r:0.1_bump",n,p_physical)).ok().unwrap()));

        let rdr_res = csv::ReaderBuilder::new().has_headers(false)
                                          .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n));

        if let Ok(rdr) = rdr_res {

            let mut rdr = rdr;

            for row in rdr.records() {
                let row = row.unwrap();
                let mut rdr2 = csv::ReaderBuilder::new().has_headers(false)
                                                   .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n)).ok().unwrap();
                    
                for row2 in rdr2.records() {

                    let row2 = row2.unwrap();
                    let g_a: String = row[3].parse().unwrap();
                    let g_b: String = row2[3].parse().unwrap();

                    if g_a == g_b {

                        let code_a = str_to_vec(&row[4]);
                        let k_a: usize = row[1].parse().unwrap();
                        
                        let code_b = str_to_vec(&row2[4]);
                        let k_b: usize = row2[1].parse().unwrap();

                        

                        let wtr = Arc::clone(&wtr);
                        
                        pool.execute_to(tx.clone(), Thunk::of(move ||{
                            let x_opt = simulate_code_quantum_classical(&code_a, k_a, g_a, &code_b, k_b, g_b, n, p_physical);
                            if let Some(x) = x_opt {
                            
                            let mut wtr = wtr.lock().unwrap();
                            wtr.write_record(&[format!("{}",x.n),
                                                format!("{}",x.k),
                                                format!("{}",x.k_a),
                                                format!("{}",x.k_b),   
                                                format!("{}",x.err.unwrap()),
                                                format!("{}",x.g_a.unwrap()), 
                                                format!("{}",x.g_b.unwrap()),format!("{:?}",code_a) , format!("{:?}",code_b)]).ok();
                                wtr.flush().ok();
                            } else {}
                        }));

                        
                        
                    }
                }

                
            }

        } else {}
        pool.join(); 
    }  

    
    

}


// pub fn landscape_from_classical_fixed_fam(n0: usize, iter: usize){
//     let p_physical = 0.2;

//     for n in n0..n0 + iter {

//         println!("at n:{}",n);

//         let wtr = Arc::new(Mutex::new(csv::Writer::from_path(&format!("/home/nouey/Projects/mnlc2/mnlc_simulations/data/landscape_from_classical_fixed_n0:{}_p:{}_r:0.1",n,p_physical)).ok().unwrap()));

//         let rdr_res = csv::ReaderBuilder::new().has_headers(false)
//                                           .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n));

//         if let Ok(rdr) = rdr_res {

//             let mut rdr = rdr;

//             rdr.records().par_bridge().for_each(|row| {
//                 let row = row.unwrap();
//                 let mut rdr2 = csv::ReaderBuilder::new().has_headers(false)
//                                                    .from_path(&format!("/home/nouey/Projects/mnlc3/mnlc_simulations/data/all_codes_5_n:{}",n)).ok().unwrap();

//                 for row2 in rdr2.records() {

//                     let row2 = row2.unwrap();
//                     let g_a: String = row[3].parse().unwrap();
//                     let g_b: String = row2[3].parse().unwrap();

//                     if g_a == g_b {

//                         let code_a = str_to_vec(&row[4]);
//                         let k_a: usize = row[1].parse().unwrap();
                        
//                         let code_b = str_to_vec(&row2[4]);
//                         let k_b: usize = row2[1].parse().unwrap();
                        

//                         let x_opt = simulate_code_quantum_classical(&code_a, k_a, g_a, &code_b, k_b, g_b, n, p_physical);

//                         if let Some(x) = x_opt {
//                             let mut wtr = wtr.lock().unwrap();
//                             wtr.write_record(&[format!("{}",x.n),
//                                                 format!("{}",x.k),
//                                                 format!("{}",x.k_a),
//                                                 format!("{}",x.k_b),   
//                                                 format!("{}",x.err.unwrap()),
//                                                 format!("{}",x.g_a.unwrap()), 
//                                                 format!("{}",x.g_b.unwrap()),row[4].to_string() , row2[4].to_string()]).ok();
//                                 wtr.flush().ok();
//                         } else {}
//                     }
//                 }
//             })

//         } else {}
//     }

    


    
    

// }

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

    ParityCheckMatrix::circulant_right(&poly, i+1)
}

pub fn conditional_simul_quantum(ag_count: Arc<Mutex<HashMap<(String,String),usize>>>, g_lim:usize, code_a: &[usize], code_b: &[usize], n: usize, p:f64) -> Option<QuantSimulRes>{

   // println!("Simulating a:{:?}, b:{:?}", code_a, code_b);

    //let matrix = ParityCheckMatrix::gbc_from_poly(code_a, code_b, n);
    let code_k = quantum_code_k(code_a, code_b, n);

    if (code_k as f64)/((2*n) as f64) >= 0.1 {

        let g_a = char_str(code_a,n);

        let g_b = char_str(code_b,n);

        let mut g_count = ag_count.lock().unwrap();

        let gs = (g_a, g_b);
        
        if !g_count.contains_key(&gs) {

            g_count.insert(gs,1);

            drop(g_count);

            return simulate_code_quantum(code_a, code_b, n, code_k,p)

        } else {
            if let Some(count) = g_count.get_mut(&gs) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return simulate_code_quantum(code_a, code_b, n, code_k,p)
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

pub fn simulate_code_quantum_classical(code_a: &[usize], k_a :usize, g_a: String, code_b: &[usize], k_b :usize, g_b: String,  n: usize, p:f64) -> Option<QuantSimulRes> {

    //println!("Started a:{:?}, b:{:?}", code_a, code_b);

    let k = quantum_code_k(code_a, code_b, n);

    if (k as f64) / ((2*n) as f64) >= 0.1 {

        let matrix = ParityCheckMatrix::gbc_from_poly(code_a, code_b, n);

        let mut decoder = QuantumErasureDecoder::new_merged(matrix, p);
        let err = decoder.simulate_until_n_events_are_found(20).get_failure_rate();
        // let err = decoder.simulate_n_iterations(10000000).get_failure_rate();
        

        //println!("Finished a:{:?}, b:{:?}", code_a, code_b);


        return Some(QuantSimulRes {
            n: 2*n,
            k_a,
            k_b,
            k,
            err: Some(err),
            cr:None,
            g_a: Some(g_a),
            g_b: Some(g_b),

        })

    } else {
        None
    }

    

}

pub fn simulate_code_quantum(code_a: &[usize], code_b: &[usize], n: usize, k: usize, p:f64) -> Option<QuantSimulRes> {

    let matrix = ParityCheckMatrix::gbc_from_poly(code_a, code_b, n);

    let mut decoder = QuantumErasureDecoder::new_merged(matrix, p);
    let err = decoder.simulate_until_n_events_are_found(20).get_failure_rate();

    let k_a = code_k(code_a, n);
    let k_b = code_k(code_b, n);

    let g_a = char_str(code_a,n);

    let g_b = char_str(code_b,n);

    println!("Finished a:{:?}, b:{:?}", code_a, code_b);


    return Some(QuantSimulRes {
        n: 2*n,
        k_a,
        k_b,
        k,
        err: Some(err),
        cr:None,
        g_a: Some(g_a),
        g_b: Some(g_b),

    })

}

pub struct QuantSimulRes {
    n: usize,
    k_a: usize,
    k_b: usize,
    k: usize,
    err: Option<f64>, //err rate
    cr: Option<usize>, // cr number
    g_a: Option<String>,
    g_b: Option<String>,
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
