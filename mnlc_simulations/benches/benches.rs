#[macro_use]

extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use believer::{ParityCheckMatrix, Decoder, EDBuilder, ClassicalSimulator, DecoderBuilder, Simulator, ErasureDecoder};
use std::time::Duration;
use std::collections::HashMap;



fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rank functions");
 
    let coef_a = vec![0, 2, 4, 6, 12];
    let n_rows = 30;
    let a = ParityCheckMatrix::circulant_right(&coef_a,n_rows);

    let dec = ErasureDecoder::new(a,0.49);
    let sim = ClassicalSimulator::new(dec);

    group.bench_function("Test simul", |b| b.iter(||  sim.simulate_until_failures_are_found_serial_mut(5)));

    println!("rate: {}", sim.simulate_until_failures_are_found_serial_mut(20).failure_rate());

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