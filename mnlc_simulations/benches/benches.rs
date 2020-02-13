#[macro_use]

extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use believer::{ParityCheckMatrix, Decoder, EDBuilder, ClassicalSimulator, DecoderBuilder, Simulator, ErasureDecoder};
use std::time::Duration;
use std::collections::HashMap;



fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rank functions");
    //group.sample_size(10);
    // group.measurement_time(Duration::from_secs(17));
    //group.bench_function("add_checks", |b| b.iter(||  add_checks(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,58,59,100,121]))));
    //group.bench_function("add_checks_mut", |b| b.iter(|| add_checks_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,58,59,100,121]), black_box(&mut vec![0,58,59,100,121]))));
   // c.bench_function("add_checks_mut2", |b| b.iter(|| repeated_addition_mut2(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,58,59,100,121]))));
   //group.bench_function("add_row_mut", |b| b.iter(|| add_rows_mut(black_box(&vec![0,15,20,28,66]),black_box(&vec![0,58,59,100,121]), black_box(&mut vec![0,58,59,100,121]))));
   //c.bench_function("add_mut", |b| b.iter(|| add_mut(black_box(&vec![0,15,20,28,66]),black_box(&vec![0,58,59,100,121]), black_box(&mut vec![0,58,59,100,121]))));
    // c.bench_function("row_add_mut", |b| b.iter(|| repeated_row_add_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66]))));
    // c.bench_function("row_add_mut2", |b| b.iter(|| repeated_row_add_mut2(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66]))));
    // c.bench_function("add_mut", |b| b.iter(|| repeated_add_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66]))));

    let coef_a = vec![0, 1, 8, 25, 112];
    let n_rows = 120;
    let a = ParityCheckMatrix::circulant_right(&coef_a,n_rows);
    let error = vec![1,2,3,4,5];
    let mut work = a.clone();

    group.bench_function("keep2", |b| b.iter(|| black_box(&a).keep2(&error)));
    group.bench_function("keep_mut", |b| b.iter(|| black_box(&a).keep_mut(&mut work, &error)));

    println!("keep2:{:?}",&a.keep2(&error));
    println!("keep_mut:{:?}",&work);


    let decoder_builder = EDBuilder::new(0.49);
    let decoder = decoder_builder.from_code(a);
    // let simulator = ClassicalSimulator::new(decoder);

    
    group.bench_function("decoder", |b| b.iter(|| black_box(&decoder).decode(&error)));
    group.bench_function("decoder_mut", |b| b.iter(|| black_box(&decoder).decode_mut(&error, &mut work)));
    
    let simulator = ClassicalSimulator::new(decoder);
    
    group.bench_function("serial", |b| b.iter(|| black_box(&simulator).simulate_until_failures_are_found_serial(5).failure_rate()));
    group.bench_function("serial_mut", |b| b.iter(|| black_box(&simulator).simulate_until_failures_are_found_serial_mut(5).failure_rate()));
    
    // group.bench_function("rank", |b| b.iter(|| black_box(&a).rank()));
    // group.bench_function("ranky2", |b| b.iter(|| black_box(&a).ranky2()));
    //group.bench_function("ranky2", |b| b.iter(|| black_box(&a).ranky2()));

    // println!("{}",a.rank());
    // println!("{}",a.ranky2());
    
    /*
    group.bench_function("hashmap_insert", |b| b.iter(|| hashmap_insert(black_box(&a))));
    group.bench_function("vec_insert", |b| b.iter(|| vec_insert(black_box(&a))));
    
    group.bench_function("hashmap_add", |b| b.iter(|| hashmap_add(black_box(&a),n_rows)));
    group.bench_function("vec_add", |b| b.iter(|| vec_add(black_box(&a),n_rows)));
    
    */

    
    // println!("{:?}",repeated_row_add_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66])));
    // println!("{:?}",repeated_row_add_mut2(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66])));
    // println!("{:?}",repeated_add_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,15,20,28,66])));
    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// fn repeated_addition(v1: &[usize], v2: &[usize]) -> Vec<usize> {
//     let mut accum = Vec::with_capacity(v1.len() + v2.len());
//     for _ in 0..100 {
//        accum = add_checks(&v1, &v2);
//     }
//     accum
// }


fn simul_serial(simulator: &ClassicalSimulator<ErasureDecoder>) {
    simulator.simulate_until_failures_are_found_serial(5).failure_rate();
}

fn simul_serial_mut(simulator: &ClassicalSimulator<ErasureDecoder>) {
    simulator.simulate_until_failures_are_found_serial_mut(5).failure_rate();
}

fn simul_par(simulator: &ClassicalSimulator<ErasureDecoder>) {
    simulator.simulate_until_failures_are_found(8,2).failure_rate();
}

fn hashmap_insert(mat: &ParityCheckMatrix) ->  HashMap<usize, Vec<usize>> {

    let n_rows = mat.n_checks();

    let bit_indices = mat.bit_indices();

    let check_ranges = mat.check_ranges();


    let mut tmp_matrix: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n_rows); 

    for i in 0..n_rows {

        tmp_matrix.insert(i,bit_indices[check_ranges[i]..check_ranges[i+1]].to_vec()); // we store the original matrix as a vector of sparse rows

    }

    tmp_matrix

}

fn hashmap_add(a: &ParityCheckMatrix, n_rows: usize) {
    let mut hashmap = hashmap_insert(&a);
    //*hashmap.get_mut(&1).unwrap() = add_checks(&hashmap.get(&0).unwrap(),&hashmap.get(&1).unwrap());  
    for i in (1..n_rows).rev(){
        *hashmap.get_mut(&1).unwrap() = add_checks(&hashmap.get(&0).unwrap(),&hashmap.get(&1).unwrap());
        hashmap.remove(&i);
    }
}

fn vec_add(a: &ParityCheckMatrix, n_rows: usize) {
    let mut vector = vec_insert(&a);
    //vector[1] = add_checks(&vector[0],&vector[1]);
    for i in 0..n_rows-1 {
        vector[0] = add_checks(&vector[1],&vector[0]);
        vector.remove(0);
    }
}

fn vec_insert(mat: &ParityCheckMatrix) -> Vec<Vec<usize>> {

    let n_rows = mat.n_checks();

    let bit_indices = mat.bit_indices();

    let check_ranges = mat.check_ranges();

    let mut tmp_matrix: Vec<Vec<usize>> = Vec::with_capacity(n_rows); 

    for i in 0..n_rows {

        tmp_matrix.push(bit_indices[check_ranges[i]..check_ranges[i+1]].to_vec()); // we store the original matrix as a vector of sparse rows

    }

    tmp_matrix

}

fn repeated_row_add_mut2(v1: &[usize], v2: &[usize])  -> Vec<usize> {
    let mut buffer: Vec<usize> = Vec::with_capacity(v1.len()); // will host result of addition
    for _ in 0..100 {
        unsafe { buffer.set_len(0)}
        add_checks_mut(&v1, &v2, &mut buffer);
    }
    buffer 
}

fn repeated_row_add_mut(v1: &[usize], v2: &[usize])  -> Vec<usize> {
    let mut buffer: Vec<usize> = Vec::with_capacity(v1.len()); // will host result of addition
    for _ in 0..100 {
        add_rows_mut(&v1, &v2, &mut buffer);
    }
    buffer 
}
/*
fn copy_to(v1: &[usize], v2: &mut Vec<usize>){
    v2.clear();
    for i in v1 {
        v2.push(*i);
    }
}*/

pub fn add_checks(check_0: &[usize], check_1: &[usize]) -> Vec<usize> {
    let mut sum = Vec::with_capacity(check_0.len() + check_1.len());
    let mut iter_0 = check_0.iter().peekable();
    let mut iter_1 = check_1.iter().peekable();
    loop {
        match (iter_0.peek(), iter_1.peek()) {
            (Some(&&v), None) => {
                sum.push(v);
                iter_0.next();
            }
            (None, Some(&&v)) => {
                sum.push(v);
                iter_1.next();
            }
            (Some(&&v_0), Some(&&v_1)) => {
                if v_0 < v_1 {
                    sum.push(v_0);
                    iter_0.next();
                } else if v_0 > v_1 {
                    sum.push(v_1);
                    iter_1.next();
                } else {
                    iter_0.next();
                    iter_1.next();
                }
            }
            (None, None) => break,
        }
    }
    sum
}


pub fn add_checks_mut(check_0: &[usize], check_1: &[usize], sum: &mut Vec<usize>){
    //let mut sum = Vec::with_capacity(check_0.len() + check_1.len());
    unsafe { sum.set_len(0) };
    let mut iter_0 = check_0.iter().peekable();
    let mut iter_1 = check_1.iter().peekable();
    loop {
        match (iter_0.peek(), iter_1.peek()) {
            (Some(&&v), None) => {
                sum.push(v);
                iter_0.next();
            }
            (None, Some(&&v)) => {
                sum.push(v);
                iter_1.next();
            }
            (Some(&&v_0), Some(&&v_1)) => {
                if v_0 < v_1 {
                    sum.push(v_0);
                    iter_0.next();
                } else if v_0 > v_1 {
                    sum.push(v_1);
                    iter_1.next();
                } else {
                    iter_0.next();
                    iter_1.next();
                }
            }
            (None, None) => break,
        }
    }
}

pub fn add_rows_mut(a_row: &[usize], b_row:&[usize], accum: &mut Vec<usize>)  {

   // println!("{:?},{:?}", a_row, b_row);
    unsafe { accum.set_len(0) };

    let mut i = 0;
    let mut j = 0;
    let mut c = 0;

        while (i < a_row.len()) && (j < b_row.len()){

            if a_row[i] == b_row[j] { // 1+1, we do nothing
                i += 1;
                j += 1;

            } else if a_row[i] > b_row[j] { 
                unsafe { accum.set_len(accum.len()+1)};
                accum[c] = b_row[j];
                c += 1;
                j += 1;
            } else {
                unsafe { accum.set_len(accum.len()+1)};
                accum[c] = (a_row[i]);
                c += 1;
                i += 1;
            }

        }

        // because a and b can have different length we take care of adding potential leftovers
        if j >= b_row.len() {
            for k in i .. a_row.len() {
                unsafe { accum.set_len(accum.len()+1)};
                accum[c] = a_row[k];
                c += 1;
            }
        } else if i >= a_row.len() {
            for k in j .. b_row.len() {
                unsafe { accum.set_len(accum.len()+1)};
                accum[c] = b_row[k];
                c += 1;
            }
        }
}

/* free style unsafe version
fn repeated_addition_mut(v1: &[usize], v2: &mut Vec<usize>) -> Vec<usize>{
    let mut accum: Vec<usize> = Vec::with_capacity(v1.len());
    for i in 0..100 {
        
        add_mut(v1, v2, &mut accum);
        for j in 0..accum.len() {
            if j < v2.capacity() {
                 v2[j] = accum[j];
            } else {
                 v2.push(accum[j]);
             }
         }
        unsafe { accum.set_len(0); }

        
    }
    v2.to_owned()
}
*/

/*

fn repeated_addition(v1: &[usize], v2: &[usize]) -> Vec<usize> {
    let mut accum = v2.to_vec();
    for _ in 0..100 {
       accum = add(v1, &accum);
    }
    accum
}

fn repeated_addition_mut(v1: &[usize], v2: &mut Vec<usize>) -> Vec<usize>{
    let mut accum: Vec<usize> = v2.to_vec();
    for i in 0..100 {
        
        transfer_to(&mut accum, v2);
        add_mut(v1, v2, &mut accum);

        //for j in 0..accum.len() {
           // if j < v2.capacity() {
            //     v2[j] = accum[j];
            //} else {
            //     panic!("Wait what");
            // }
        // }
        //unsafe { accum.set_len(0); }

        
    }
    accum.to_owned()
}

fn transfer_to(v1: &mut Vec<usize>, v2: &mut Vec<usize>){
    for i in 0..v1.len() {
        v2[i] = v1[i];
    }
    v2.truncate(v1.len());
    v1.clear();
}

fn repeated_addition_mut2(v1: &[usize], v2: &[usize]) -> Vec<usize>{
    let mut mut_v2 = v2.to_vec();
    for _ in 0..100 {
      add_mut2(v1, &mut mut_v2);
    }
    mut_v2
}

fn add(v1: &[usize], v2: &[usize]) -> Vec<usize> {
    let mut accum: Vec<usize> = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        accum.push(v1[i]+v2[i]);
    }
    accum
}

fn add_mut(v1: &[usize], v2: &[usize], buffer: &mut Vec<usize>){
    unsafe{ buffer.set_len(v1.len()); }
    for i in 0..v1.len() {
        buffer[i] = (v1[i] + v2[i]);
    }
}

fn add_mut2(v1: &[usize], v2: &mut Vec<usize>){
    for i in 0..v1.len() {
        v2[i] = v1[i] + v2[i];
    }
}
*/

fn add_mut(v1: &[usize], v2: &[usize], buffer: &mut Vec<usize>){
    
    for i in 0..v1.len() {
        buffer[i] = (v1[i] + v2[i]);
    }
}

fn repeated_add_mut(v1: &[usize], v2: &[usize]) -> Vec<usize>{
    let mut accum: Vec<usize> = Vec::with_capacity(v1.len());
    unsafe{ accum.set_len(v1.len()); }
    for _ in 0..100 {
        
        add_mut(v1, v2, &mut accum);
        
    }
    accum
}