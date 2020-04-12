use believer::{ParityCheckMatrix};

use std::cmp::Eq;
use std::hash::Hash;

use pagenumber::HEA;

use std::cmp::Ordering::*;

use mnlc_simulations::*;
use std::collections::HashMap;


use std::sync::{Arc, Mutex};


//51, 54,



fn main(){

    // let n = 9;
    // let k = 2;
    // let iter = 20;
    // let ws = vec![2,3,4,5];
    // test_family_across_weight_min_pages(n,k,iter,&ws);
    let (v1, v2) = poly_to_edgelist_pg(&[0, 4, 6], 8);//0,1,2  3

    println!("{}",HEA(&v2,&v1));


}



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