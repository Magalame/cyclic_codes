use believer::{ParityCheckMatrix,add_checks,add_checks_mut};
use believer::{CSSEDBuilder,EDBuilder,ClassicalSimulator,DecoderBuilder,Simulator};
use std::time::{Duration, Instant};

use rayon::prelude::*;
use crate::graphs::*;
use crate::qc::*;
use crate::flint::*;

use std::cmp::Ordering::*;

use mnlc_simulations::*;
use std::error::Error;

//51, 54,

fn main(){

    test_family(93,1);

}

pub fn binary_search(data: &[usize], target: &usize) -> bool {

    if target < &data[0] {
        return false
    }

    let mut high = data.len()-1;
    let mut low = 0;
    
 
    while low <= high {
        let mid = (low + high) / 2;

        match data[mid].cmp(target) {
            Less => low = mid + 1,
            Greater => high = mid - 1,
            Equal => return true,
        };
    }
    return false
}


// pub fn binary_search(data: &[usize], target: &usize) -> bool {
//     let mut high = data.len();
//     let mut low = 0;
//     let mut mid = high / 2;
 
//     while low < high {
//         match target.cmp(&data[mid]) {
//             Less => high = mid - 1,
//             Greater => low = mid + 1,
//             Equal => return true,
//         };
//         mid = (high + low) / 2;
//     }
//     return false
// }




fn mgg(x: i32, y: i32, n:i32, res: &mut Vec<(i32,i32)>)  {

    res[0] = ((x + 2*y).rem_euclid(n),y);
    res[1] = ((x - 2*y).rem_euclid(n),y);
    
    res[2] = ((x+2*y+1).rem_euclid(n),y);
    res[3] = ((x-2*y-1).rem_euclid(n),y);

    res[4] = (x,(y+2*x).rem_euclid(n));
    res[5] = (x,(y-2*x).rem_euclid(n));

    res[6] = (x, (y+2*x+1).rem_euclid(n));
    res[7] = (x, (y-2*x-1).rem_euclid(n));

    //println!("({:},{:}):{:?}",x,y,res);
}

fn mgg2(x: i32, y: i32, n:i32, res: &mut Vec<(i32,i32)>)  {

    res[0] = ((x + 1).rem_euclid(n),y);
    res[1] = ((x - 1).rem_euclid(n),y);
    
    res[2] = ((x+y).rem_euclid(n),y);
    res[3] = ((x-y).rem_euclid(n),y);

    res[4] = (x,(y+1).rem_euclid(n));
    res[5] = (x,(y-1).rem_euclid(n));

    res[6] = (x, (y+x).rem_euclid(n));
    res[7] = (x, (y-x).rem_euclid(n));

    //println!("({:},{:}):{:?}",x,y,res);
}

use std::convert::TryInto;

fn mgg_to_sparse(mgg: Vec<Vec<i32>>, n: usize) -> ParityCheckMatrix{

    let mut out: Vec<Vec<usize>> = Vec::with_capacity(n*n);
    
    for i in 0..n*n {
        let mut row: Vec<usize> = Vec::with_capacity(8);

        for j in 0..n*n {
            if mgg[i][j] == 1 {
                row.push(j);
            }
        }

        out.push(row);

    }

    ParityCheckMatrix::new(out, n*n)

}

fn fill_mgg(n: i32) -> Vec<Vec<i32>> {

    let mut out: Vec<Vec<i32>> = vec![vec![0; (n*n).try_into().unwrap()]; (n*n).try_into().unwrap()];
    let mut tmp: Vec<(i32,i32)> = vec![(0,0); 8];

    for x in 0..n {
        for y in 0..n {
            mgg2(x, y, n, &mut tmp);
            let input: usize = (x*n + y).try_into().unwrap();
            for (x_i,y_i) in &tmp {
                //println!("{}",(x*n + y));
                //println!("{}",(x_i*n+y_i));
                let input2: usize = (x_i*n+y_i).try_into().unwrap();
               out[input][input2] = 1;
            //out[input2][input] = 1;
            }
        }
    }

    out

}

fn print_mgg(display: &Vec<Vec<i32>>){
    for row in display {
        println!("{:?}",row);
    }
}