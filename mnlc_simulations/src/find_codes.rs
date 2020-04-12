use std::cmp::{max,min};
use believer::{ParityCheckMatrix, EDBuilder,ClassicalSimulator,DecoderBuilder,Simulator,add_checks_mut};
use rayon::prelude::*;
use crate::graphs::*;
use crate::qc::*;
use crate::flint::*;
use std::ops::Range;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, MutexGuard};
use gcd::Gcd;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Drain;
use pagenumber::*;

use std::cmp::Eq;
use std::hash::Hash;

use std::sync::mpsc::channel;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};


pub fn test_family_across_weight_min_pages(n0: usize,k0: usize, iter: usize, ws:&[usize]){
   
    let lim_per_g = 5;

    let min_page = HashMap::new();

    let m_min_page = SharedHashMap::new(min_page);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/min_page/fam_n0:{}_k0:{}_min_pg",n0,k0)).ok().unwrap();


    for i in 1..iter+1 {

        let n = n0*i;

        println!("at n:{}",n);

        for w in ws {

           println!("\tat w:{}",w);

            let g_count: HashMap<String, usize> = HashMap::new();
            let m_g_count = Arc::new(Mutex::new(g_count));
            
            let k = k0*i;

            let mut poly_index: Vec<usize> = Vec::with_capacity(*w);             

            init(&mut poly_index, *w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            let n_workers = 8;
            let mut pool = Pool::<ThunkWorker<()>>::new(n_workers);

            let (tx, rx) = channel();

            let mut count = 0;

            for code in poly {
                let a_g_count = Arc::clone(&m_g_count);
                let a_min_page = m_min_page.clone();
                pool.execute_to(tx.clone(), Thunk::of(move ||{
                    conditional_simul_min_page(a_g_count, a_min_page, lim_per_g, &code, n, k);
                })); 
                count += 1;
            }

            

            pool.join();


        }

        m_min_page.with_drain(|(k,simul_res)| {

            let simul_res = simul_res.lock().unwrap();
            println!("before");
            wtr.write_record(&[format!("{}",simul_res.n),format!("{}",simul_res.k), format!("{}",simul_res.err.expect("no err")),format!("{}",simul_res.cr.expect("no cr")),format!("{}",simul_res.g.as_ref().expect("no g")),format!("{:?}",simul_res.coef.as_ref().expect("no coef"))]).ok();
            println!("after");
            wtr.flush().ok();
        }

        );

    }

    


}

pub fn test_family_fixed(n0: usize,k0:usize, iter: usize){
    
    let lim_per_g = 5;

    //let m_wtr = Arc::new(Mutex::new(wtr));

    for w in &[3] {

        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam{}_n0:{}_k0:{}_pg",w,n0,k0)).ok().unwrap();


        for i in 1..iter {

            let g_count: HashMap<String, usize> = HashMap::new();
            let m_g_count = Arc::new(Mutex::new(g_count));

            let mut poly_index: Vec<usize> = Vec::with_capacity(*w); 
            
            let n = n0*i;
            let k = k0*i;

            println!("at n={}",n);

            

            init(&mut poly_index, *w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            let n_workers = 8;
            let pool = Pool::<ThunkWorker<Option<SimulRes>>>::new(n_workers);

            let (tx, rx) = channel();

            let mut count = 0;

            for code in poly {
                let a_g_count = Arc::clone(&m_g_count);
                pool.execute_to(tx.clone(), Thunk::of(move ||{
                    let x = conditional_simul(a_g_count, lim_per_g, &code, n, k);
                    x
                }));
                count += 1;
            }

            

            for res in rx.iter().take(count) {
                if let Some(sres) = res {
                    wtr.write_record(&[format!("{}",n),format!("{}",sres.k), format!("{}",sres.err.unwrap()),format!("{}",sres.cr.unwrap()),format!("{}",sres.g.unwrap()),format!("{:?}",sres.coef.unwrap())]).ok();
                    wtr.flush().ok();
                }
            }


        }

    }

    
    

}


pub fn test_all(n0: usize, iter: usize){
    
    let lim_per_g = 5;

    
    //let m_wtr = Arc::new(Mutex::new(wtr));

    for w in &[5] {


        for i in 0..iter {

            let g_count: HashMap<String, usize> = HashMap::new();
            let m_g_count = Arc::new(Mutex::new(g_count));

            let mut poly_index: Vec<usize> = Vec::with_capacity(*w); 
            
            let n = n0 + i;

            let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/all_codes_5_n:{}",n)).ok().unwrap();

            init(&mut poly_index, *w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            let n_workers = 8;
            let pool = Pool::<ThunkWorker<Option<SimulRes>>>::new(n_workers);

            let (tx, rx) = channel();

            let mut count = 0;

            for code in poly {
                let a_g_count = Arc::clone(&m_g_count);
                pool.execute_to(tx.clone(), Thunk::of(move ||{
                    let x = conditional_simul_all_k(a_g_count, lim_per_g, &code, n);
                    x
                }));
                count += 1;
            }

            

            for res in rx.iter().take(count) {
                if let Some(sres) = res {
                    wtr.write_record(&[format!("{}",n),format!("{}",sres.k), format!("{}",sres.err.unwrap()),format!("{}",sres.g.unwrap()),format!("{:?}",sres.coef.unwrap())]).ok();
                    wtr.flush().ok();
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


pub fn conditional_simul(ag_count: Arc<Mutex<HashMap<String,usize>>>, g_lim:usize, code: &Vec<usize>, n: usize, target_k: usize) -> Option<SimulRes> {

    let code_k = code_k(code, n);

    if target_k == code_k {

        let g = char_str(code,n);

        let mut g_count = ag_count.lock().unwrap();

        
        if !g_count.contains_key(&g) {

            g_count.insert(g,1);

            drop(g_count);

            return Some(simulate_code(code, n, target_k))

        } else {
            if let Some(count) = g_count.get_mut(&g) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return Some(simulate_code(code, n, target_k))
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

pub fn conditional_simul_min_page(ag_count: Arc<Mutex<HashMap<String,usize>>>,amin_page: SharedHashMap<String, SimulRes>, g_lim:usize, code: &Vec<usize>, n: usize, k: usize){

    let actual_k = code_k(code, n);

    if actual_k == k {
    
        let g = char_str(code, n);

        let mut g_count = ag_count.lock().unwrap();

        if g_count.contains_key(&g) {

            let mut count = g_count.get_mut(&g).unwrap();

            if *count < g_lim {
                    *count += 1;

                    drop(g_count);

                    // let (v1, v2) = poly_to_edgelist(code,n);

                    // let new_pg = cr(&v1, &v2);

                    let (edges, vertices) = poly_to_edgelist_pg(code,n);
                    let new_pg = HEA(&vertices, &edges);

                    let simul_res = amin_page.get(&g).expect(&format!("g {:?} not found in simul_res HashMap",&g));
                    let mut simul_res = simul_res.lock().unwrap();

                    if let Some(cur_min_pg) = simul_res.cr {
                        if new_pg < cur_min_pg {
                            simul_res.cr = Some(new_pg);
                            simul_res.coef = Some(code.to_owned());
                        }
                    } else {
                        simul_res.cr = Some(new_pg);
                        simul_res.coef = Some(code.to_owned());
                    }
                    // println!("for g:{}, cur page:{}",g,cur_min_pg);

                    // println!("new_pg:{}, old_pg:{}",new_pg,cur_min_pg);

                    

                    // println!("new cur:{}",simul_res.cr.unwrap());
            }

        } else { 

            g_count.insert(g.clone(),1);

            drop(g_count);

            let simul_res = amin_page.get(&g);

            if let Some(simul_res) = simul_res {

                let (edges, vertices) = poly_to_edgelist_pg(code,n);

                let new_pg = HEA(&vertices, &edges);

                let mut simul_res = simul_res.lock().unwrap();

                if let Some(cur_min_pg) = simul_res.cr {
                    if new_pg < cur_min_pg {
                        simul_res.cr = Some(new_pg);
                        simul_res.coef = Some(code.to_owned());
                    }
                } else {
                    simul_res.cr = Some(new_pg);
                    simul_res.coef = Some(code.to_owned());
                }
                

            } else {

                amin_page.insert(g.clone(),SimulRes::empty());

                let res = simulate_code(code, n, k); // we simulate the code

                let simul_res = amin_page.get(&g).unwrap();
                let mut simul_res = simul_res.lock().unwrap();

                if let Some(cr) = simul_res.cr { //since during the simulation time, something might have been written, we need to check we never overwrite
                  
                  simul_res.n = n;
                  simul_res.k = k;
                  simul_res.g = Some(g.clone());
                  simul_res.coef = Some(code.to_owned());
                  simul_res.err = res.err; // this part will always be needed

                  if res.cr < simul_res.cr {
                      simul_res.cr = res.cr;
                      simul_res.coef = Some(code.to_owned());
                  }

                } else {
                    *simul_res = res;  
                }

                
                
            }


                      


        } 
    
    } 

}

pub fn conditional_simul_all_k(ag_count: Arc<Mutex<HashMap<String,usize>>>, g_lim:usize, code: &Vec<usize>, n: usize) -> Option<SimulRes>{

    
    let code_k = code_k(code, n);

    if (code_k as f64)/(n as f64) >= 0.1 {

        let g = char_str(code,n);

        let mut g_count = ag_count.lock().unwrap();

        
        if !g_count.contains_key(&g) {

            g_count.insert(g,1);

            drop(g_count);

            return Some(simulate_code(code, n, code_k))

        } else {
            if let Some(count) = g_count.get_mut(&g) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return Some(simulate_code(code, n, code_k))
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

            return Some(simulate_code(code, n, code_k))

        } else {
            if let Some(count) = g_count.get_mut(&g) {

                if *count < g_lim {
                    *count += 1;

                    drop(count);
                    drop(g_count);

                    return Some(simulate_code(code, n, code_k))
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

pub fn simulate_code(code: &Vec<usize>, n: usize, target_k: usize) -> SimulRes {

    // println!("Started: {:?}",code);

    let decoder_builder = EDBuilder::new(0.49);
    let err = get_err(code, n, &decoder_builder);
    let g = char_str(code, n);

    println!("Started: {:?}, g:{}",code,g);

    // println!("{},{:?}",n,code);
    // let (v1, v2) = poly_to_edgelist(code,n);
    // let cr = cr(&v1, &v2);
    let (edges, vertices) = poly_to_edgelist_pg(code,n);

    let cr = HEA(&vertices, &edges);
    // println!("ed {:?}",code);
    //println!("enter");
    //println!("out");
    //let cr = code_cr(code, n);
    
    //let mul = multiplicity(&code, n);

    //println!("Ended: {:?}",code);
    return SimulRes {
        n,
        k:target_k,
        err: Some(err),
        cr: Some(cr),
        g: Some(g),
        f: None,
        mul: None,
        coef: Some(code.to_vec())
    }

}



pub struct SimulRes {
    n: usize,
    k: usize,
    err: Option<f64>, //err rate
    cr: Option<usize>, // cr number
    g: Option<String>,
    f: Option<String>,
    mul: Option<f64>,
    coef: Option<Vec<usize>>
}

impl SimulRes {
    fn empty() -> Self {
        Self {
            n:0,
            k:0,
            err: None,
            cr: None,
            g: None,
            f: None,
            mul: None,
            coef: None
        }
    }
}

pub fn all_first_codes(max_n: usize) {
    for w in 2..7 {

        println!("at w:{}",w);

        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/all_first_codes_w:{}",w)).ok().unwrap();

        let mut codes: Vec<(usize, usize)> = Vec::new();

        for n in w..max_n{

            println!("\tat n:{}",n);

            let mut poly_index: Vec<usize> = Vec::with_capacity(w);            

            init(&mut poly_index, w);

            let poly = Poly{
                indexes: Some(poly_index),
                n
            };

            'next_code: for code in poly {
                let k = code_k(&code, n);

                if k > 0 {
                    for (n0,k0) in &codes {

                        if k%k0 == 0 && n%n0 == 0 && k/k0 == n/n0{ // if actually belongs to a family
                            continue 'next_code
                        }

                    }

                    wtr.write_record(&[format!("{}",n),format!("{}",k)]).ok();
                    wtr.flush().ok();

                    codes.push((n,k));


                }

            }


        }
    }
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

pub struct Poly {
    indexes: Option<Vec<usize>>,
    n: usize
}

impl Poly {
    pub fn new(indexes: Option<Vec<usize>>, n: usize) -> Self{
        Poly {
            indexes,
            n
        }
    }
}

pub fn init(vec: &mut Vec<usize>, w:usize){

    vec.clear();

    for i in 0..w {
        vec.push(i);
    }
}

impl Iterator for Poly {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let indexes = self.indexes.as_mut().unwrap();
        let w = indexes.len();

        //let out = indexes.clone();

        
        if indexes[1] < n - w + 1 { //&& indexes[w-1] < n 
            for i in 1..w {
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

pub struct SharedHashMap<K: Hash + Eq,V>{
    a_hashmap: Arc<Mutex<HashMap<K,Arc<Mutex<V>>>>>
}

impl <K: Hash + Eq,V> SharedHashMap<K,V>{
    fn new(hashmap: HashMap<K, Arc<Mutex<V>>>) -> Self {
        Self{
            a_hashmap: Arc::new(Mutex::new(hashmap))
        }
    }

    fn get(&self, k:&K) -> Option<Arc<Mutex<V>>> {
        let m_h = &self.a_hashmap.lock().unwrap();
        let value = m_h.get(k);
        if let Some(refr) = value {
            Some(Arc::clone(refr))
        } else {
            None
        }
    }

    fn insert(&self, k:K, v: V) {
        let mut m_h = self.a_hashmap.lock().unwrap();
        m_h.insert(k,Arc::new(Mutex::new(v)));
    }

    fn clone(&self) -> Self {
        Self {
            a_hashmap: Arc::clone(&self.a_hashmap)
        }
    }

    fn with_drain<F>(&self, mut func: F) 
    where F: FnMut((K,Arc<Mutex<V>>))
    {
        let mut m_h = self.a_hashmap.lock().unwrap();
        m_h.drain().map(|(k,v)| func((k,v)) ).count();
    }

}