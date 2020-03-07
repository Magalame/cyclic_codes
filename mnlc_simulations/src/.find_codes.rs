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

fn find_max_contrib(v: &[usize], u: &[usize]) -> usize {

    assert_eq!(v.len(),u.len());

    let original_cr = cr(&v,&u);

    let res: usize = (0..v.len()).map(|i| {

        let mut new_v = v.to_owned();
        new_v.swap_remove(i);
        let mut new_u = u.to_owned();
        new_u.swap_remove(i);

        let c = original_cr - cr(&new_v,&new_u);
        c*c

    }).max().unwrap();

    res

}





pub fn error_and_cr_5(range_of_ns: Range<usize>)  {

    range_of_ns.into_par_iter().for_each( 
        |n| {

        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}", n)).ok().unwrap();
        for i_0 in 1..(n-3) { 
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        let indices = vec![0,i_0,i_1,i_2,i_3];

                        let matrix = ParityCheckMatrix::circulant_right(&indices, n);
                        let k = n-matrix.rank();

                        let decoder_builder = EDBuilder::new(0.4);
                        let decoder = decoder_builder.from_code(matrix);
                        let simulator = ClassicalSimulator::new(decoder);
                        let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                        let (v1,v2) = poly_to_edgelist(&indices,n);
                        let cr_nb = cr(&v1,&v2);

                        wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();

    
                        

                    }
                }
            }
            
        }
       wtr.flush().ok();
    }

    );
}

pub fn find_configs_5(n: usize, k: usize){

    (1..n-3).into_par_iter().for_each( 
        |i_0| {
            println!("i_0:{}",i_0);
            let mut indices = vec![0, 0, 0, 0, 0];
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        indices[1] = i_0;
                        indices[2] = i_1;
                        indices[3] = i_2;
                        indices[4] = i_3;
                        
                        let matrix = ParityCheckMatrix::circulant_right(&indices, n);

                        let rank = matrix.rank();
                        if rank <= n - k {
                            println!("Poly: 0,{},{},{},{}, k:{}",i_0,i_1,i_2,i_3,n-rank);
                        }

                    }
                }
            }
        }

    )


}

pub fn error_and_cr_5_par(n: usize)  {

    let decoder_builder = EDBuilder::new(0.49);

    let shared_counter = Arc::new(Mutex::new(0));

    (1..n-3).into_par_iter().for_each( 
        |i_0| {

       // println!("Running i0:{}/{}",i_0,n-4);
        let local_counter = Arc::clone(&shared_counter);
        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}_5/i0_{}",n, i_0)).ok().unwrap();
        
            'outer: for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {
                //println!("started ({},{})",i_0,i_1);
                let counter = local_counter.lock().unwrap();

                if *counter > 50 {
                    drop(counter);
                    break 'outer; // we break all the loops
                }

                drop(counter);
               
                
                //let start_k = Instant::now();
                let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                let k = n-matrix.rank();
                
                //let duration_k = start_k.elapsed();

                //println!("k:{:?}",duration_k);
                
                if k > 0 {
                    //println!("k:{},{},{},{}",k,0,i_0,i_1);
                    //let start_err = Instant::now();
                    let mut mut_counter = local_counter.lock().unwrap();
                    *mut_counter += 1;
                    drop(mut_counter);
                    
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found(1,10).failure_rate();
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2,i_3,n);
                    let cr_nb = cr(&v1,&v2);
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();
                    //println!("ended ({},{})",i_0,i_1);
                }

                    }
                }

            }
            
        
        wtr.flush().ok();

        println!("Finished i0:{}/{}",i_0,n-4);
        }

    );
}


pub fn error_and_cr_5_par_k(n: usize, target_k: usize)  {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/err_and_cr_n:{}_k:{}",n,target_k)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);

    let shared_counter = Arc::new(Mutex::new(0));
    let shared_writer = Arc::new(Mutex::new(wtr));

    (1..n-3).into_par_iter().for_each( 
        |i_0| {

       // println!("Running i0:{}/{}",i_0,n-4);
        let local_counter = Arc::clone(&shared_counter);
        let local_writer = Arc::clone(&shared_writer);
        
            'outer: for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {
                //println!("started ({},{})",i_0,i_1);
                let counter = local_counter.lock().unwrap();

                if *counter > 400 {
                    drop(counter);
                    break 'outer; // we break all the loops
                }

                drop(counter);
               
                
                //let start_k = Instant::now();
                let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                let k = n-matrix.rank();
                
                //let duration_k = start_k.elapsed();

                //println!("k:{:?}",duration_k);
                
                if k == target_k{
                    //println!("k:{},{},{},{}",k,0,i_0,i_1);
                    //let start_err = Instant::now();
                    let mut mut_counter = local_counter.lock().unwrap();
                    *mut_counter += 1;
                    drop(mut_counter);
                    
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(10).failure_rate();
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2,i_3,n);
                    //let cr_nb =  find_max_contrib(&v1,&v2);
                    let cr_nb =  cr(&v1,&v2);
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    let mut wtr = local_writer.lock().unwrap();
                    wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();
                    wtr.flush().ok();
                    //println!("ended ({},{})",i_0,i_1);
                }

                    }
                }

            }
            
        
        

        println!("Finished i0:{}/{}",i_0,n-4);
        }

    );
}



pub fn error_and_cr_par_random(w:usize, r: usize, n: usize)  {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/err_and_cccr_n:{}_r:{}_w:{}",n,r,w)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);

    let shared_counter = Arc::new(Mutex::new(0));
    let shared_writer = Arc::new(Mutex::new(wtr));

    (0..8 as usize).into_par_iter().for_each( 
        |i_0| {

       // println!("Running i0:{}/{}",i_0,n-4);
        loop { 
        let local_counter = Arc::clone(&shared_counter);
        let local_writer = Arc::clone(&shared_writer);
        
        let counter = local_counter.lock().unwrap();

        if *counter > 2000 {
            drop(counter);
            break
        } else {
            drop(counter);
        }

        let matrix = ParityCheckMatrix::rand_ldpc(r, n, w);
        let k = n-matrix.rank();
                
                if true {
                    //println!("k:{},{},{},{}",k,0,i_0,i_1);
                    //let start_err = Instant::now();
                    let mut mut_counter = local_counter.lock().unwrap();
                    *mut_counter += 1;
                    drop(mut_counter);

                    let (v1,v2) = matrix.to_edge_list();
                    //let cr_nb =  find_max_contrib(&v1,&v2);
                    let cr_nb =  find_max_contrib(&v1,&v2);
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(10).failure_rate();
                    
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    let mut wtr = local_writer.lock().unwrap();
                    wtr.write_record(&[format!("{}",k),format!("{}",err_rate), format!("{}",cr_nb) ]).ok();
                    wtr.flush().ok();
                    //println!("ended ({},{})",i_0,i_1);
                }      

        
        }
        }

    );
}

pub fn error_and_cr_par_random_bicy(w: usize, n: usize)  {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/err_and_cr_n_bicy:{}",n)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);

    let shared_counter = Arc::new(Mutex::new(0));
    let shared_writer = Arc::new(Mutex::new(wtr));

    (0..8 as usize).into_par_iter().for_each( 
        |i_0| {

       // println!("Running i0:{}/{}",i_0,n-4);
        loop { 
        let local_counter = Arc::clone(&shared_counter);
        let local_writer = Arc::clone(&shared_writer);
        
        let counter = local_counter.lock().unwrap();

        if *counter > 2000 {
            drop(counter);
            break
        } else {
            drop(counter);
        }

        let matrix = ParityCheckMatrix::rand_bicy(w,n);
        //let matrix = ParityCheckMatrix::rand_ldpc(24, 30, 5);

        let k = n-matrix.rank();
                
                if k == 4 {
                   // println!("{:?}",matrix);
                    //let start_err = Instant::now();
                    let mut mut_counter = local_counter.lock().unwrap();
                    *mut_counter += 1;
                    drop(mut_counter);

                    let (v1,v2) = matrix.to_edge_list();
                    //let cr_nb =  find_max_contrib(&v1,&v2);
                    let cr_nb =  cr(&v1,&v2);
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(10).failure_rate();
                    
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    let mut wtr = local_writer.lock().unwrap();
                    wtr.write_record(&[format!("{}",k),format!("{}",err_rate), format!("{}",cr_nb) ]).ok();
                    wtr.flush().ok();
                    //println!("ended ({},{})",i_0,i_1);
                }      

        
        }
        }

    );
}

pub fn error_and_cr_par_random_bicy_k(w: usize, k:usize, n: usize)  {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/err_and_cr_n_bicy_n{}_k{}",n,k)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);

    let shared_counter = Arc::new(Mutex::new(0));
    let shared_writer = Arc::new(Mutex::new(wtr));

    (0..8 as usize).into_par_iter().for_each( 
        |i_0| {

       // println!("Running i0:{}/{}",i_0,n-4);
        loop { 
        let local_counter = Arc::clone(&shared_counter);
        let local_writer = Arc::clone(&shared_writer);
        
        let counter = local_counter.lock().unwrap();

        if *counter > 2000 {
            drop(counter);
            break
        } else {
            drop(counter);
        }

        let matrix = ParityCheckMatrix::rand_bicy(w,n);
        //let matrix = ParityCheckMatrix::rand_ldpc(24, 30, 5);

        let k = n-matrix.rank();
                
                if k == k {
                   // println!("{:?}",matrix);
                    //let start_err = Instant::now();
                    let mut mut_counter = local_counter.lock().unwrap();
                    *mut_counter += 1;
                    drop(mut_counter);

                    let (v1,v2) = matrix.to_edge_list();
                    //let cr_nb =  find_max_contrib(&v1,&v2);
                    let cr_nb =  cr(&v1,&v2);
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(10).failure_rate();
                    
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    let mut wtr = local_writer.lock().unwrap();
                    wtr.write_record(&[format!("{}",k),format!("{}",err_rate), format!("{}",cr_nb) ]).ok();
                    wtr.flush().ok();
                    //println!("ended ({},{})",i_0,i_1);
                }      

        
        }
        }

    );
}
// pub fn error_and_cr_5_par(n: usize)  {

//     let decoder_builder = EDBuilder::new(0.49);
//     (1..n-3).into_par_iter().for_each( 
//         |i_0| {
//        // println!("Running i0:{}/{}",i_0,n-4);
//         let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}_5/i0_{}",n, i_0)).ok().unwrap();
//             for i_1 in (i_0+1)..(n-2) {
//                 for i_2 in (i_1+1)..(n-1) {
//                     for i_3 in (i_2+1)..n {
//                 //println!("started ({},{})",i_0,i_1);         
//                 //let start_k = Instant::now();
//                 let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
//                 let k = n-matrix.rank();           
//                 //let duration_k = start_k.elapsed();
//                 //println!("k:{:?}",duration_k);             
//                 if k > 0 {
//                     //println!("k:{},{},{},{}",k,0,i_0,i_1);
//                     //let start_err = Instant::now();               
//                     let decoder = decoder_builder.from_code(matrix);
//                     let simulator = ClassicalSimulator::new(decoder);
//                     let err_rate = simulator.simulate_until_failures_are_found(1,10).failure_rate();
//                     //let duration_err = start_err.elapsed();
//                     //println!("err:{:?}",duration_err);
//                     //let start_cr = Instant::now();
//                     let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2,i_3,n);
//                     let cr_nb = cr(&v1,&v2);
//                     //let duration_cr = start_cr.elapsed();
//                     //println!("cr:{:?}",duration_cr);
//                     wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();
//                     //println!("ended ({},{})",i_0,i_1);
//                 }
//                     }
//                 }
//             }        
//         wtr.flush().ok();
//         println!("Finished i0:{}/{}",i_0,n-4);
//         }
//     );
// }


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




pub fn count_code_5(n: usize, target_k: usize) -> usize{

    let count_found: usize =  
        (1..n-3).into_par_iter().map( 
        |i_0| {
            let mut count = 0;
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {
                
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        
                        let k = n-matrix.rank();

                        if k == target_k {
                            count += 1;
                            //println!("vec!{:?},", indices);
                        }
                    }
                }               
            }
            count
        }

    ).sum();
    
    count_found   


}

pub fn count_all_codes_5(n: usize) -> usize{

     
        (1..n-3).into_iter().map( 
        |i_0| {
            
            let mut count = 0;
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {
                
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        
                        let k = n-matrix.rank();

                        if k > 0 {
                            count += 1;
                        }
                    }
                }               
            }
            count
        }

    ).sum();
    
    count_found   


}

pub fn all_codes_5() {

    
    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/all_codes_5")).ok().unwrap();

    for n in 4..500 {
        let mut buffer: Vec<(usize, usize)> = Vec::with_capacity(n);
        let mut codes = HashSet::new();
        for i_0 in 1..n-3 {
            //println!("i_0:{}",i_0);
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    ((i_2+1)..n).into_par_iter().map( |i_3| {
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        
                        let k = n-matrix.rank();

                        (n,k)
                    }).collect_into_vec(&mut buffer);

                    for (n,k) in &buffer {
                        if *k != 0 && k.gcd(*n) == 1 {
                            let k0 = *k;
                            let n0 = *n;
                            if !codes.contains(&(n0,k0)){
                                codes.insert((n0,k0));
                                wtr.write_record(&[format!("{}",n),format!("{}",k)]).ok();
                            }
                            
                        }
                    }
                    wtr.flush().ok();


                }               
            }
        }
    }
    


}

pub fn mem_test(){

    let mm = vec![0, 16, 76, 80, 100];

    (0..500).into_par_iter().for_each( |_| {
        for _ in 0..50000 {
            let my_str = char_str(&mm, 120);
        }
        
    } );

}

// let counter = Arc::new(Mutex::new(0));
// let counter = Arc::clone(&counter);
// let mut shared_counter = counter.lock().unwrap();

pub fn all_first_codes_5() {
    


    
    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/all_first_codes_5")).ok().unwrap();

    for n in 51..500 {
        println!("At n:{}",n);
        let mut buffer: Vec<(usize, usize)> = Vec::with_capacity(n);
        let mcodes = Arc::new(Mutex::new(HashSet::new()));

        (1..n-3).into_par_iter().for_each( |i_0| {

            let acodes = Arc::clone(&mcodes);

            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    for i_3 in (i_2+1)..n {
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        
                        let k = n-matrix.rank();

                        if k > 0 {

                        let mut found = false;

                        let mut shared_codes = acodes.lock().unwrap();

                        for (n0,k0) in shared_codes.iter() {

                            if k%k0 == 0 && n%n0 == 0 {
                                found = true;
                            }

                        }

                        if !found {
                            (*shared_codes).insert((n,k));
                        }

                        }

                        
                    }

                    


                }               
            }
        });

        for (n,k) in mcodes.lock().unwrap().iter() {
            if n.gcd(*k) == 1 {
                wtr.write_record(&[format!("{}",n),format!("{}",k),format!("prime")]).ok();
                wtr.flush().ok();
            } else {
                wtr.write_record(&[format!("{}",n),format!("{}",k),format!("not prime")]).ok();
                wtr.flush().ok();
            }
            

        }

    }


    
    


}

pub fn all_codes_3() {

    
    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/all_codes_3")).ok().unwrap();

    for n in 4..500 {
        let mut buffer: Vec<(usize, usize)> = Vec::with_capacity(n);
        let mut codes = HashSet::new();
        for i_0 in 1..n-1 {
            //println!("i_0:{}",i_0);
             ((i_0+1)..n).into_par_iter().map( |i_1| {
                        let matrix = ParityCheckMatrix::circulant_right_better_3(i_0,i_1, n);
                        
                        let k = n-matrix.rank();

                        (n,k)
                    }).collect_into_vec(&mut buffer);

                    for (n,k) in &buffer {
                        if *k != 0 && k.gcd(*n) == 1 {
                            let k0 = *k;
                            let n0 = *n;
                            if !codes.contains(&(n0,k0)){
                                codes.insert((n0,k0));
                                wtr.write_record(&[format!("{}",n),format!("{}",k)]).ok();
                            }
                            
                        }
                    }
                    wtr.flush().ok();


        }
    }
    


}

pub fn find_configs_3(n: usize, k: usize){

    (1..n-1).into_par_iter().for_each( 
        |i_0| {
            println!("i_0:{}",i_0);
            let mut indices = vec![0, 0, 0];
            for i_1 in (i_0+1)..n {
                
                        indices[1] = i_0;
                        indices[2] = i_1;
                        
                        let matrix = ParityCheckMatrix::circulant_right(&indices, n);
                        
                        let rank = matrix.rank();
                        if rank <= n - k {
                            println!("Poly: 0,{},{}, k:{}",i_0,i_1,n-rank);
                        }

                
            }
        }

    )


}

pub fn find_max_k_3(n: usize)->usize{

    let mut max_max = 0;

    if let Some(max_found) =  
        (1..n-1).into_par_iter().map( 
        |i_0| {
            //println!("i_0:{}",i_0);
            let mut indices = vec![0, 0, 0];
            let mut max_k = 0;
            for i_1 in (i_0+1)..n {
                
                indices[1] = i_0;
                indices[2] = i_1;
                        
                let matrix = ParityCheckMatrix::circulant_right_better(&indices, n);
                        
                let k = n-matrix.rank();

                if k > max_k  {
                    max_k = k;
                }
                                        
            }
            max_k
        }

    ).max() {
    
    //println!("n={}, max k= {}", n, max_found);
    max_max = max_found

    }

    max_max
}



pub fn count_code_3(n: usize, target_k: usize) -> usize{

    let count_found: usize =  
        (1..n-1).into_par_iter().map( 
        |i_0| {
            //println!("i_0:{}",i_0);
            let mut indices = vec![0, 0, 0];
            let mut count = 0;
            for i_1 in (i_0+1)..n {
                
                indices[1] = i_0;
                indices[2] = i_1;
                        
                let matrix = ParityCheckMatrix::circulant_right_better(&indices, n);
                        
                let k = n-matrix.rank();

                if k  == target_k {
                    count += 1;
                    //println!("vec!{:?},", indices);
                }
                                        
            }
            count
        }

    ).sum();
    
    count_found   


}

pub fn count_all_codes_3(n: usize) -> usize{

    let count_found: usize =  
        (1..n-1).into_par_iter().map( 
        |i_0| {
            //println!("i_0:{}",i_0);
            let mut indices = vec![0, 0, 0];
            let mut count = 0;
            for i_1 in (i_0+1)..n {
                
                indices[1] = i_0;
                indices[2] = i_1;
                        
                let matrix = ParityCheckMatrix::circulant_right_better(&indices, n);
                        
                let k = n-matrix.rank();

                if k > 0 {
                    count += 1;
                }
                                        
            }
            count
        }

    ).sum();
    
    count_found   


}

pub fn test_family_3(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.5);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam3_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    for i in 1..max_n+1 {
        let n = n0*i;
        println!("at n={}",n);
        let k = k0*i;
        
        for i_0 in 1..n-1 {
            let results: Vec<f64> = ((i_0+1)..n).into_par_iter().map(|i_1| {

                let matrix = ParityCheckMatrix::circulant_right_better_3(i_0, i_1, n);
                let mut err_rate = -1.0;
                if n-matrix.rank() == k {
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    err_rate = simulator.simulate_until_failures_are_found(2,5).failure_rate();
                    
                }
                err_rate
            }

            ).filter(|x| *x != -1.0).collect();

            for err_rate in results {
                wtr.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate) ]).ok();
            }
            wtr.flush().ok();
        }


    }


}

pub fn test_family_3_mul(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.5);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam3_mul_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    for i in 1..max_n+1 {
        let n = n0*i;
        println!("at n={}",n);
        let k = k0*i;
        
        for i_0 in 1..n-1 {
            let results: Vec<(f64,u32)> = ((i_0+1)..n).into_par_iter().map(|i_1| {

                let matrix = ParityCheckMatrix::circulant_right_better_3(i_0, i_1, n);
                let mut err_rate = -1.0;
                if n-matrix.rank() == k {
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    err_rate = simulator.simulate_until_failures_are_found(2,5).failure_rate();
                    
                }
                (err_rate, multiplicity(vec![0,i_0, i_1])) 
            }

            ).filter(|(x,_)| *x != -1.0).collect();

            for (err_rate,mul) in results {
                wtr.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate), format!("{}",mul) ]).ok();
            }
            wtr.flush().ok();
        }


    }


}

pub fn test_family_5_wcr(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.49);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_cr_n0{}_k0{}", n0, k0)).ok().unwrap();
    


    for i in 1..max_n+1 {
        let n = n0*i;
        let k = k0*i;
        let mut res: Vec<(usize,f64,usize)> = Vec::with_capacity(n);
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-1 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 20 {
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        let mut err_rate = 0.0;
                        let mut cr_nb = 0;
                        //println!("{}/{}", code_k,k);
                        if code_k == k {
                            
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                            let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2, i_3,n);
                            cr_nb = cr(&v1,&v2);
                        }

                        (code_k,err_rate, cr_nb)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate, cr_nb) in &res {
                        if *code_k == k {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate),format!("{}",cr_nb)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

    }


}

pub fn test_family_5_wmul(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.49);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_Test_mul_n0{}_k0{}", n0, k0)).ok().unwrap();
    


    for i in 1..max_n+1 {
        let n = n0*i;
        let k = k0*i;
        let mut res: Vec<(usize,f64,usize)> = Vec::with_capacity(n);
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-1 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 20 {
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        let mut err_rate = 0.0;
                        let mut cr_nb = 0.;
                        //println!("{}/{}", code_k,k);
                        if code_k == k {
                            
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                            cr_nb = multiplicity(vec![0,i_0,i_1,i_2, i_3], n);
                        }

                        (code_k,err_rate, cr_nb)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate, cr_nb) in &res {
                        if *code_k == k {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate),format!("{}",cr_nb)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

    }


}

pub fn test_family_5_fixed_char(n0: usize, k0: usize, max_n: usize ) {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_test_all_new_c_n0{}_k0{}",n0,k0)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);
    
    let mwriter = Arc::new(Mutex::new(wtr));

    for i in 1..max_n+1 {
        let n = n0*i;
        let k = k0*i;
        let mcodes = Arc::new(Mutex::new(HashSet::new()));
        println!("at n={}",n);
        
        (1..n-1).into_par_iter().for_each(|i_0| {

            let awriter = Arc::clone(&mwriter);
            let acodes = Arc::clone(&mcodes);
            let mut indexes: Vec<usize> = vec![0; 5]; 

            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        indexes[1] = i_0;
                        indexes[2] = i_1;
                        indexes[3] = i_2;
                        indexes[4] = i_3;
                        
                        let char_string = char_str(&indexes, n);

                        let mut codes = acodes.lock().unwrap(); 
                        
                        if !codes.contains(&char_string) {

                            codes.insert(char_string.clone()); 

                            drop(codes);
                            
                            let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                            let code_k = n-matrix.rank();

                            if code_k == k {
                                
                                
                                let decoder = decoder_builder.from_code(matrix);
                                let simulator = ClassicalSimulator::new(decoder);
                                let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                                let mut shared_writer = awriter.lock().unwrap();
                                shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",&err_rate), format!("{}",&char_string) ]).ok();
                                shared_writer.flush().ok();
                            } 

                                  

                        }
                    }
                }
            }

        }); 

    }
}

pub fn test_family_5_mem_test(n0: usize, k0: usize, max_n: usize ) {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_mem_test_n0{}_k0{}",n0,k0)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);
    
    let mwriter = Arc::new(Mutex::new(wtr));

    for i in 1..max_n+1 {
        let n = n0*i;
        let k = k0*i;
        let mcodes = Arc::new(Mutex::new(HashSet::new()));
        println!("at n={}",n);
        
        (1..n-1).into_par_iter().for_each(|i_0| {

            let awriter = Arc::clone(&mwriter);
            let acodes = Arc::clone(&mcodes);
            let mut indexes: Vec<usize> = vec![0; 5]; 

            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        indexes[1] = i_0;
                        indexes[2] = i_1;
                        indexes[3] = i_2;
                        indexes[4] = i_3;
                        
                        let char_string = char_str(&indexes, n);

                        let mut codes = acodes.lock().unwrap(); 
                        
                        if !codes.contains(&char_string) {

                            codes.insert(char_string.clone()); 

                            drop(codes);
                            
                            let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                            let code_k = n-matrix.rank();

                            if code_k == k {
                                
                                
                                let decoder = decoder_builder.from_code(matrix);
                                let simulator = ClassicalSimulator::new(decoder);
                                let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                                let mut shared_writer = awriter.lock().unwrap();
                                shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",&err_rate), format!("{}",&char_string) ]).ok();
                                shared_writer.flush().ok();
                            } 

                                  

                        }
                    }
                }
            }

        }); 

    }
}

pub fn test_char_vs_cr_family_5(n0: usize, k0: usize, max_n: usize ) {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_cvc_n0{}_k0{}",n0,k0)).ok().unwrap();
    let mwtr = Arc::new(Mutex::new(wtr));

    let decoder_builder = EDBuilder::new(0.49);
    
    (1..max_n+1).into_par_iter().for_each(|i| {

        let n = n0*i;
        let k = k0*i;

        let mut indexes: Vec<usize> = vec![0; 5];

        let mut codes = HashMap::new();

        let awriter = Arc::clone(&mwtr);

         for i_0 in 1..n-1 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        indexes[1] = i_0;
                        indexes[2] = i_1;
                        indexes[3] = i_2;
                        indexes[4] = i_3;
                        
                        let char_string = char_str(&indexes, n);

                        if !codes.contains_key(&char_string) {

                            
                            let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                            let code_k = n-matrix.rank();

                            if code_k == k {
                                
                                let (v1,v2) = poly_to_edgelist_5(i_0, i_1, i_2, i_3, n);
                                let cr = cr(&v1, &v2);

                                let mut shared_writer = awriter.lock().unwrap();
                                shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",cr), format!("{}",&char_string) ]).ok();
                                shared_writer.flush().ok();
                            } 
                            codes.insert(char_string,1);       

                        } else {

                            if let Some(count) = codes.get_mut(&char_string) {

                                if *count < 5 {
                                    let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                                    let code_k = n-matrix.rank();

                                    if code_k == k {

                                        let (v1,v2) = poly_to_edgelist_5(i_0, i_1, i_2, i_3, n);
                                        let cr = cr(&v1, &v2);

                                        let mut shared_writer = awriter.lock().unwrap();
                                        shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",cr), format!("{}",&char_string) ]).ok();
                                        shared_writer.flush().ok();
                                    }

                                    *count += 1;
                                } 

                            }

                        }

                    }

                } 


            }

        } 

    });


}

pub fn test_family_5_fixed_char_2(n0: usize, k0: usize, max_n: usize ) {

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_fc_n0{}_k0{}",n0,k0)).ok().unwrap();

    let decoder_builder = EDBuilder::new(0.49);
    
    for i in 1..max_n+1 {
        let n = n0*i;
        let k = k0*i;
        
        let mut codes = HashSet::new();
        let mut indexes = vec![0;5];
        
        for i_0 in 1..n-1 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {


                        indexes[1] = i_0;
                        indexes[2] = i_1;
                        indexes[3] = i_2;
                        indexes[4] = i_3;

                        let char_string = char_str(&indexes, n);

                        if !codes.contains(&char_string) {

                            codes.insert(char_string);
                            let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                            let code_k = n-matrix.rank();

                            if code_k == k {
                                let decoder = decoder_builder.from_code(matrix);
                                let simulator = ClassicalSimulator::new(decoder);
                                let err_rate = simulator.simulate_until_failures_are_found(8,2).failure_rate();

                                wtr.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate), format!("{:?}",&indexes) ]).ok();
                                wtr.flush().ok();
                            }        

                        }

                    }
                }
            }
        } 
    }
}


pub fn test_family_5(n0: usize, k0: usize, max_n: usize ) {

    let wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_test_fcwpf_n0{}_k0{}",n0,k0)).ok().unwrap();
    let mwtr = Arc::new(Mutex::new(wtr));

    
    
    for i in 1..max_n+1 {

        let n = n0*i;
        let k = k0*i;

        

        let codes = HashMap::new();
        let mcodes = Arc::new(Mutex::new(codes));
        

        (1..n-1).into_par_iter().for_each(|i_0| {

            let awriter = Arc::clone(&mwtr);
            let acodes = Arc::clone(&mcodes);

            let mut indexes: Vec<usize> = vec![0; 5];

            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {
                    for i_3 in (i_2+1)..n {

                        indexes[1] = i_0;
                        indexes[2] = i_1;
                        indexes[3] = i_2;
                        indexes[4] = i_3;
                        
                        let char_string = char_str(&indexes, n);
                        let pf_string = prefactor_str(&indexes, n);

                        let mut codes = acodes.lock().unwrap();

                        if !codes.contains_key(&char_string) {

                            drop(codes);

                            
                            let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                            let code_k = n-matrix.rank();

                            if code_k == k {
                                let decoder = decoder_builder.from_code(matrix);
                                let simulator = ClassicalSimulator::new(decoder);
                                let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                                let mut shared_writer = awriter.lock().unwrap();
                                shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate), format!("{}",&char_string), format!("{}",&pf_string) ]).ok();
                                shared_writer.flush().ok();
                            } 

                            let mut codes = acodes.lock().unwrap();
                            codes.insert(char_string,1);       

                        } else {

                            if let Some(count) = codes.get_mut(&char_string) {

                                if *count < 5 {

                                    *count += 1;

                                    drop(count);
                                    drop(codes);

                                    let matrix = ParityCheckMatrix::circulant_right_better(&indexes, n);
                                    let code_k = n-matrix.rank();

                                    if code_k == k {

                                        let decoder = decoder_builder.from_code(matrix);
                                        let simulator = ClassicalSimulator::new(decoder);
                                        let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();

                                        let mut shared_writer = awriter.lock().unwrap();
                                        shared_writer.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate), format!("{}",&char_string), format!("{}",&pf_string)  ]).ok();
                                        shared_writer.flush().ok();
                                    }

                                    
                                } 

                            }

                        }

                    }

                } 


            }

        }); 

    }


}



pub fn test_family_3_range(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam3_range_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,f64,usize)> = Vec::with_capacity(n + max_n);
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-1 {
            if count <= 10 {
            ((i_0+1)..n).into_par_iter().map( |i_1| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_3(i_0,i_1, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            //err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                            err_rate = simulator.simulate_until_failures_are_found(5,2).failure_rate();
                            
                        }

                        let mul= multiplicity(vec![0,i_0,i_1], n);

                        (code_k,err_rate,mul)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate, mul) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate), format!("{}",mul)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                   
                }
            }
            

        }

        n += 1;

    }


}


pub fn test_family_3_range_test(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam3_range_n0{}_k0{}_test", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,f64)> = Vec::with_capacity(n + max_n);
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-1 {
            if count <= 1 {
            ((i_0+1)..n).into_par_iter().map( |i_1| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_3(i_0,i_1, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.;
                        let code_ratio = (code_k as f64)/(n as f64);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1  {
                            err_rate = 1.;
                        }
                        
                        //println!("{}/{}", code_k,k);
                        // if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            
                        //     let decoder = decoder_builder.from_code(matrix);
                        //     let simulator = ClassicalSimulator::new(decoder);
                        //     //err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                        //     err_rate = simulator.simulate_until_failures_are_found(5,2).failure_rate();
                            
                        // }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                   
                }
            }
            

        }

        n += 1;

    }


}

// test is there just to see how far we can go

pub fn test_family_5_range_test(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_test_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = 151;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,f64)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;
        let mut tried = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 1 && tried < 10000 {
                    tried += 1;
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {
                            err_rate = 1.;
                            
            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += 1;

    }


}

 

pub fn test_family_5_range(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,f64)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            //err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                            err_rate = simulator.simulate_until_failures_are_found(5,2).failure_rate();
                            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += 1;

    }


}

pub fn test_family_5_range_wcr(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,f64,usize)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.0;
                        let mut cr_nb = 0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            //err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                            err_rate = simulator.simulate_until_failures_are_found(5,2).failure_rate();
                            let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2, i_3,n);
                            cr_nb = cr(&v1,&v2);
                        }

                        (code_k,err_rate, cr_nb)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate, cr_nb) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate), format!("{}",cr_nb)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += 1;

    }


}

pub fn test_family_5_range_random_wcr(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_random_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(Vec<(usize,f64,usize)>)> = Vec::with_capacity(10);
    
    for n in n0..max_n {

        println!("At n={}",n);


        (0..8).into_par_iter().map(|_| {

            let mut trials = 0;
            let mut nb_found = 0;
            let mut local_res: Vec<(usize,f64,usize)> = Vec::with_capacity(2);

            while nb_found < 2&& trials < 1000 {
                let checks = ParityCheckMatrix::rand_checks(5, n);
                let matrix = ParityCheckMatrix::circulant_right_better(&checks, n);

                let code_k = n-matrix.rank();
                
                let code_ratio = (code_k as f64)/(n as f64);
                
                if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(10).failure_rate();
                    // let err_rate = simulator.simulate_until_failures_are_found(5,2).failure_rate();
                    let (v1,v2) = poly_to_edgelist(&checks,n);
                    let cr_nb = cr(&v1,&v2);

                    local_res.push((code_k,err_rate, cr_nb));

                    nb_found += 1;
                }

                trials +=1;

            }

            local_res


        } ).collect_into_vec(&mut res);

        for row in &res {

            for (code_k, err_rate, cr_nb) in row {
                if *err_rate != 0. {
                    wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate), format!("{}",cr_nb)]).ok();
                }
            }
        }
        wtr.flush().ok();


    }
    
    


}

pub fn test_family_5_range_mul(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_mul_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,usize)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            //err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                            err_rate = multiplicity(vec![0,i_0,i_1,i_2,i_3], n);
                            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += 1;

    }


}

pub fn test_family_5_range_cr(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_range_cr_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,usize)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if target_ratio - 0.05 < code_ratio && code_ratio < target_ratio + 0.05 && code_k.gcd(n) == 1 {

                            
                            let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2, i_3,n);
                            err_rate = cr(&v1,&v2);
                            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0 {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += 1;

    }


}

pub fn test_family_5_cr(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_cr_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,usize)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if code_ratio == target_ratio  {

                            
                            let (v1,v2) = poly_to_edgelist_5(i_0,i_1,i_2, i_3,n);
                            err_rate = cr(&v1,&v2);
                            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0 {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += n0;

    }


}

pub fn test_family_5_mul(n0: usize, k0: usize, max_n: usize ) {

    let decoder_builder = EDBuilder::new(0.499);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_mul_n0{}_k0{}", n0, k0)).ok().unwrap();
    
    let mut n = n0;
    let target_ratio = (k0 as f64)/( n0 as f64);
    let mut res: Vec<(usize,usize)> = Vec::with_capacity(min(10,n*max_n));
    for i in 1..max_n+1 {
        
        
        let mut count = 0;

        println!("at n={}",n);
        
        for i_0 in 1..n-3 {
            for i_1 in (i_0+1)..(n-2) {
                for i_2 in (i_1+1)..(n-1) {

                    if count < 10 {
                    let nb_codes = n-(i_2+1);
                    //println!("here, stuck at nb={}",nb_codes);
                    ((i_2+1)..n).into_par_iter().map( |i_3| {                        
                        
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0,i_1,i_2,i_3, n);
                        let err_rate = -1.0;
                        let code_k = n-matrix.rank();
                        
                        let mut err_rate = 0.;
                        let code_ratio = (code_k as f64)/(n as f64);
                        //println!("{}/{}", code_k,k);
                        if code_ratio == target_ratio  {

                            
                            err_rate = multiplicity(vec![0,i_0,i_1,i_2,i_3], n);
                            
                        }

                        (code_k,err_rate)
                        
                    }).collect_into_vec(&mut res);

                    for (code_k, err_rate) in &res {
                        if *err_rate != 0. {

                            //println!("{},{},{},counted",code_k,k,err_rate);

                            wtr.write_record(&[format!("{}",n),format!("{}",code_k), format!("{}",err_rate)]).ok();
                            wtr.flush().ok();
                            count += 1
                        } 
            
                    }
                    }
                }
            }

        }

        n += n0;

    }


}



pub fn test_family_5_2(n0: usize, k0: usize, max_n: usize ) {


    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam5_n0{}_k0{}", n0, k0)).ok().unwrap();

    for i in 1..max_n+1 {
        let n = n0*i;
        println!("at n={}",n);
        let k = k0*i;

        let counter = Arc::new(Mutex::new(0));

        let res: Vec<(usize, usize, f64)> = (1..n-1).into_par_iter().flat_map(|i_0| {
            let counter = Arc::clone(&counter);
            ((i_0+1)..(n-2)).into_par_iter().flat_map( move |i_1| {
                let counter = Arc::clone(&counter);
                ((i_1+1)..(n-1)).into_par_iter().flat_map( move |i_2| {
                    let counter = Arc::clone(&counter);
                    ((i_2+1)..n).into_par_iter().filter_map( move |i_3| {

                        
                        
                        let decoder_builder = EDBuilder::new(0.49);
                        let matrix = ParityCheckMatrix::circulant_right_better_5(i_0, i_1, i_2, i_3, n);

                        let mut shared_counter = counter.lock().unwrap();
                        if *shared_counter < 50 && n-matrix.rank() == k {
                            *shared_counter += 1;
                            drop(shared_counter);
                            let decoder = decoder_builder.from_code(matrix);
                            let simulator = ClassicalSimulator::new(decoder);
                            let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                            Some((n,k,err_rate))
                        } else {
                            drop(shared_counter);
                            None
                        }
                    })
                })
            })

        }).collect();

        for (n,k,err_rate) in res {
            wtr.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate)]).ok();
            wtr.flush().ok();
        }

    }
 

}



pub fn test_myfamily(n0: usize, k0: usize, max_n: usize ) {

    let codes: Vec<Vec<usize>> = vec![vec![0, 1, 2],
vec![0, 8, 13],
vec![0, 1, 14],
vec![0, 2, 13],
vec![0, 4, 11],
vec![0, 5, 13],
vec![0, 13, 14],
vec![0, 4, 14]];

    let decoder_builder = EDBuilder::new(0.4);

    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/fam3_n0{}_k0{}", n0, k0)).ok().unwrap();

    for i in (1..max_n+1).step_by(2) {

        let n = n0*i;
        println!("at n={}",n);
        let k = k0*i;
        
        let results: Vec<f64> = codes.par_iter().map(|code| {

        let matrix = ParityCheckMatrix::circulant_right_better_3(code[1]*i, code[2]*i, n);

        let decoder = decoder_builder.from_code(matrix);
        let simulator = ClassicalSimulator::new(decoder);
        let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();      
        err_rate
        }
        ).collect();

            for err_rate in results {
                wtr.write_record(&[format!("{}",n),format!("{}",k), format!("{}",err_rate) ]).ok();
            }
            wtr.flush().ok();
        


    }


}

// pub fn distance_and_cr_3(n: usize)  {

//     (1..n-1).into_par_iter().for_each( 
//         |i_0| {
//             println!("Running i0:{}/{}",i_0,n-2);
//             let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}/i0_{}",n, i_0)).ok().unwrap();
            
//             for i_1 in (i_0+1)..n {
//                 let mut min_res_modulo = usize::max_value();

//                 for m in 1..(i_1+2) {

//                     let temp_res = max( i_0/m + i_0%m, i_1/m + i_1%m);

//                     min_res_modulo = min(temp_res,min_res_modulo);

//                 }

//                 let (v1,v2) = poly_to_edgelist(&[0,i_0,i_1],n);

//                 let cr_nb = cr(&v1,&v2);

//                 wtr.write_record(&[format!("{}",cr_nb),format!("{}",min_res_modulo) ]).ok();

    
                   


//             }
//             wtr.flush().ok();

//             println!("Finished i0:{}/{}",i_0,n-2);

//         }

//     );

// }

pub fn error_and_cr_3(range_of_ns: Range<usize>)  {

    range_of_ns.into_par_iter().for_each( 
        |n| {

        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}", n)).ok().unwrap();
        for i_0 in 1..(n-1) { 
            println!("i_0:{}",i_0);
            for i_1 in (i_0+1)..n {
                let indices = vec![0,i_0,i_1];

                let matrix = ParityCheckMatrix::circulant_right_better(&indices, n);
                let k = n-matrix.rank();

                let decoder_builder = EDBuilder::new(0.4);
                let decoder = decoder_builder.from_code(matrix);
                let simulator = ClassicalSimulator::new(decoder);
                let err_rate = simulator.simulate_until_failures_are_found(8,2).failure_rate();

                let (v1,v2) = poly_to_edgelist(&indices,n);
                let cr_nb = cr(&v1,&v2);

                wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();

            }
            
        }
       wtr.flush().ok();
    }

    );
}

pub fn error_and_cr_3_par(n: usize)  {

    let decoder_builder = EDBuilder::new(0.4);

    (1..n-1).into_par_iter().for_each( 
        |i_0| {

        //println!("Running i0:{}/{}",i_0,n-2);

        let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}/i0_{}",n, i_0)).ok().unwrap();
        let mut count = 0;
        
            for i_1 in (i_0+1)..n {
                //println!("started ({},{})",i_0,i_1);

                if count < 10 {
                
                //let start_k = Instant::now();
                let matrix = ParityCheckMatrix::circulant_right_better_3(i_0,i_1, n);
                let k = n-matrix.rank();
                
                //let duration_k = start_k.elapsed();

                //println!("k:{:?}",duration_k);
                
                if k > 0 {
                    //println!("k:{},{},{},{}",k,0,i_0,i_1);
                    //
                    count += 1;
                    //let start_err = Instant::now();
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(16).failure_rate();
                    //let duration_err = start_err.elapsed();
                    //

                    //let start_cr = Instant::now();
                    let (v1,v2) = poly_to_edgelist_3(i_0,i_1,n);
                    let cr_nb = cr(&v1,&v2);
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);

                    wtr.write_record(&[format!("{}",cr_nb),format!("{}",k),format!("{}",err_rate) ]).ok();
                    
                    //println!("rank:{:?},err:{:?}, {},{},{}",duration_k, duration_err,0,i_0,i_1);
                    //println!("ended ({},{})",i_0,i_1);
                }

                }
                
                
            }
            
        
        wtr.flush().ok();

        println!("Finished i0:{}/{}",i_0,n-2);
        }

    );
}

pub fn error_and_cr_3_par_2(n: usize)  {

    let decoder_builder = EDBuilder::new(0.49);
    let mut results: Vec<Option<(usize, usize, f64)>> = Vec::with_capacity(n);
    let mut wtr = csv::Writer::from_path(&format!("/home/nouey/Projects/simul/mnlc_simulations/data/{}/cr_k_err",n)).ok().unwrap();

    for i_0 in 1..n-1 {

        println!("Running i0:{}/{}",i_0,n-2);
        
        
            ((i_0+1)..n ).into_par_iter().map( |i_1| {
                //println!("started ({},{})",i_0,i_1);
                
                //let start_k = Instant::now();
                let matrix = ParityCheckMatrix::circulant_right_better_3(i_0,i_1, n);
                let k = n-matrix.rank();
                
                //let duration_k = start_k.elapsed();

                //println!("k:{:?}",duration_k);
                
                if k > 0 {
                    //println!("k:{},{},{},{}",k,0,i_0,i_1);
                    //let start_err = Instant::now();
                    
                    let decoder = decoder_builder.from_code(matrix);
                    let simulator = ClassicalSimulator::new(decoder);
                    let err_rate = simulator.simulate_until_failures_are_found_serial(20).failure_rate();
                    //let duration_err = start_err.elapsed();
                    //println!("err:{:?}",duration_err);

                    //let start_cr = Instant::now();
                    let (v1,v2) = poly_to_edgelist_3(i_0,i_1,n);
                    let cr_nb = cr(&v1,&v2);
                    //let duration_cr = start_cr.elapsed();
                    //println!("cr:{:?}",duration_cr);
                    
                    
                    //println!("ended ({},{})",i_0,i_1);
                    Some((cr_nb, k, err_rate))

                } else {
                    None
                }
                

            }).collect_into_vec(&mut results);

        for res in &results {
            if let Some((cr,k,err)) = res {
                wtr.write_record(&[format!("{}",cr),format!("{}",k),format!("{}",err) ]).ok();
            }
        }

        wtr.flush().ok();
    }


}


pub fn k_graph(n: usize){
    let mut v1:Vec<usize> = Vec::new();
    let mut v2:Vec<usize> = Vec::new();
    for i in 0..n {
        for j in 0..i {
            v1.push(i);
            v2.push(j);
        }
        for j in (i+1)..n {
            v1.push(i);
            v2.push(j);
        }
    }
    println!("n:{}, cr:{}", n, cr(&v1,&v2));
}

