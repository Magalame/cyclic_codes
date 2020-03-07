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
    let mut tmp_vec = Vec::with_capacity(10);
    let mut in2 = Vec::with_capacity(10);
    in2.push(0);
    in2.push(58);
    in2.push(59);
    in2.push(100);
    in2.push(121);
    group.bench_function("add_checks", |b| b.iter(||  add_checks(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,58,59,100,121]))));
    group.bench_function("add_checks_mut", |b| b.iter(|| add_checks_mut(black_box(&vec![0,15,20,28,66]), black_box(&vec![0,58,59,100,121]), &mut tmp_vec)));

    group.bench_function("test_add", |b| b.iter(||  test_add()));
    group.bench_function("test_add_mut", |b| b.iter(|| test_add_mut(&mut in2, &mut tmp_vec)));

    let coef_a = vec![0, 1, 14, 22, 70];
    let n_rows = 84;
    let a = ParityCheckMatrix::circulant_right_better(&coef_a,n_rows);
    let b = ParityCheckMatrix::circulant_right_better(&coef_a,n_rows);

    // let decoder_builder = EDBuilder::new(0.49);
    // let decoder = decoder_builder.from_code(b);

    // let error = decoder.random_error();
    // let mut work = a.clone();
    // let mut work2 = a.clone();

    // group.bench_function("keep", |b| b.iter(|| black_box(&a).keep(&error)));
    // group.bench_function("keep_mut", |b| b.iter(|| black_box(&a).keep_mut(&mut work, &error)));
    // group.bench_function("keep_mut2", |b| b.iter(|| black_box(&a).keep_mut2(&mut work2, &error)));

    // println!("keep:{:?}",&a.keep(&error));
    // println!("keep_mut:{:?}",&work);
    // println!("keep_mut2:{:?}",&work2);
    
    // group.bench_function("decoder", |b| b.iter(|| black_box(&decoder).decode(&error)));
    // group.bench_function("decoder_mut", |b| b.iter(|| black_box(&decoder).decode_mut(&error, &mut work)));
    
    // let simulator = ClassicalSimulator::new(decoder);
    
    // group.bench_function("serial", |b| b.iter(|| black_box(&simulator).simulate_until_failures_are_found_serial(5).failure_rate()));
    // group.bench_function("serial_mut", |b| b.iter(|| black_box(&simulator).simulate_until_failures_are_found_serial_mut(5).failure_rate()));
    
    let mut work_pcm = a.tmp_rank_pcm();
    let mut work_pcm2 = a.tmp_rank_pcm();

    let mut tmp_sum: Vec<usize> = Vec::with_capacity(n_rows);

    // group.bench_function("rank", |b| b.iter(|| black_box(&a).rank()));
    // group.bench_function("rank_mut", |b| b.iter(|| black_box(&a).rank_mut(black_box(&mut work_pcm))));
    // group.bench_function("rank_mut2", |b| b.iter(|| black_box(&a).rank_mut2(black_box(&mut work_pcm2),black_box(&mut tmp_sum))));

    // println!("{}",a.rank());
    // println!("{}",a.rank_mut(&mut work_pcm));
    // println!("{}",a.rank_mut2(&mut work_pcm2, &mut tmp_sum));
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

// fn repeated_row_add_mut(v1: &[usize], v2: &[usize])  -> Vec<usize> {
//     let mut buffer: Vec<usize> = Vec::with_capacity(v1.len()); // will host result of addition
//     for _ in 0..100 {
//         add_rows_mut(&v1, &v2, &mut buffer);
//     }
//     buffer 
// }
pub fn test_add(){
    let in1 = vec![0,15,20,28,66];
    let in2 = vec![0,58,59,100,121];
    for _ in 0..1000{
        add_checks(&in1,&in2);
    }
}

pub fn test_add_mut(in2: &mut Vec<usize>, tmp_sum: &mut Vec<usize>){

    //let mut tmp_sum = Vec::with_capacity(10);
    let in1 = vec![0,15,20,28,66];
   // let mut in2 = Vec::with_capacity(10);
    unsafe{ in2.set_len(5)}
    in2[0] = 0;
    in2[1] = 58;
    in2[2] = 59;
    in2[3] = 100;
    in2[4] = 121;

    for _ in 0..1000 {
        add_checks_mut(&in1, &in2, tmp_sum);
        transfer_to(tmp_sum, in2);
        tmp_sum.clear();
        //unsafe{ tmp_sum.set_len(0)}
        unsafe{ in2.set_len(5)}

        in2[0] = 0;
        in2[1]= 58;
        in2[2] = 59;
        in2[3] = 100;
        in2[4] = 121;
    }

    
}

fn transfer_to(v1: &[usize], v2: &mut Vec<usize>){
    // if v2.capacity() < v1.len() {
    //    println!("cap:{}, len:{}",v2.capacity(), v1.len());
    //    println!("{:?}",v1);
    // }
    unsafe{ v2.set_len(v1.len())}
    for i in 0..v1.len() {
        v2[i] = v1[i];
    }
}

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
    sum.clear();
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