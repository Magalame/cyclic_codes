use believer::{ParityCheckMatrix,add_checks,add_checks_mut};
use believer::{CSSEDBuilder,EDBuilder,ClassicalSimulator,DecoderBuilder,Simulator};
use std::time::{Duration, Instant};

use rayon::prelude::*;
use crate::graphs::*;
use crate::qc::*;
use crate::flint::*;

use mnlc_simulations::*;
use std::error::Error;



fn main() -> Result<(),Box<dyn Error>> {
    //  let parity_check = ParityCheckMatrix::new(
    //       vec![
    //           vec![0, 1, 2],
    //           vec![1, 2],
    //           vec![2],
    //       ],
    //       3
    //   );

    // println!("{:?}",parity_check.keep(&[1]).rank());

    //   let parity_check2 = ParityCheckMatrix::new(
    //       vec![
    //           vec![0],
    //           vec![0,1],
    //           vec![0,1,2],
    //       ],
    //       3
    //   );

    //  let parity_check3 = ParityCheckMatrix::new(
    //       vec![
    //           vec![0, 2],
    //           vec![1],
    //           vec![2],
    //       ],
    //       3
    //   );

    //  let parity_check4 = ParityCheckMatrix::new(
    //       vec![
    //           vec![0,1,2,3],
    //           vec![1,2,3,4],
    //           vec![2,3,4,5],
    //       ],
    //       6
    //   );

    //  println!("Matrix is:{:?}",parity_check);

    //  println!("slice 0:{:?}",parity_check.check(0).unwrap());
    //  println!("slice 1:{:?}",parity_check.check(1).unwrap());

    //  println!("Transpose is:{:?}",parity_check.transpose());


    //  println!("Matrix2 is:{:?}",parity_check2);
    //  println!("Matrix3 is:{:?}",parity_check3);

    //  println!("Matrix^2 is:{:?}",parity_check.mult(&parity_check));

    //  println!("Matrix4 is:{:?}",parity_check4);
    //  println!("Right concat is:{:?}",parity_check.right_concat(&parity_check2));

    //  println!("Diag concat is:{:?}",parity_check.diag_concat(&parity_check2));

    //  println!("Permu matrix is:{:?}",ParityCheckMatrix::permu_matrix(2));

    //  println!("matrix + matrix is:{:?}",parity_check.add(&parity_check));

    //  println!("matrix + matrix2 is:{:?}",parity_check.add(&parity_check2));

    //  println!("circulant empty is:{:?}",ParityCheckMatrix::circulant(&vec![],2));

    //   println!("circulant 0 is:{:?}",ParityCheckMatrix::circulant(&vec![0],2));
    //   println!("circulant 0 is:{:?}",ParityCheckMatrix::circulant_down(&vec![0],2));

    // println!("circulant 0 is:{:?}",ParityCheckMatrix::circulant_right(&vec![0],2));
    // println!("circulant 0 is:{:?}",ParityCheckMatrix::circulant_right_better(&vec![0],2));

    //   println!("circulant 1 is:{:?}",ParityCheckMatrix::circulant(&vec![1],2));

    //   println!("circulant 1 is:{:?}",ParityCheckMatrix::circulant_down(&vec![1],2));

    // println!("circulant 1 is:{:?}",ParityCheckMatrix::circulant_right(&vec![1],2));
    // println!("circulant 1 is:{:?}",ParityCheckMatrix::circulant_right_better(&vec![1],2));

    //   println!("circulant 0+1 is:{:?}",ParityCheckMatrix::circulant(&vec![0,1],2));

    //   println!("circulant 0+1 is:{:?}",ParityCheckMatrix::circulant_down(&vec![0,1],2));

    // println!("circulant 0+1 is:{:?}",ParityCheckMatrix::circulant_right(&vec![0,1],2));
    // println!("circulant 0+1 is:{:?}",ParityCheckMatrix::circulant_right_better(&vec![0,1],2));

    //   println!("circulant 2 (ident) is:{:?}",ParityCheckMatrix::circulant(&vec![2],2));

    //   println!("circulant 2 (ident) is:{:?}",ParityCheckMatrix::circulant_down(&vec![2],2));

    // println!("circulant 2 (ident) is:{:?}",ParityCheckMatrix::circulant_right(&vec![2],2));
    // println!("circulant 2 (ident) is:{:?}",ParityCheckMatrix::circulant_right_better(&vec![2],2));
     
    // let col: Vec<_> = parity_check.checks_iter().collect();

    //  println!("Collect is:{:?}",col);


    //  let test_gen = GBCGenerator::new(127,
    //                               3,
    //                               10,
    //                               None);

    //  let decoder_builder = CSSEDBuilder::new(0.3);


    //  let perf = test_gen.simulate(decoder_builder,5,20_000);

    //  for (code,fail) in perf {
    //     println!("Res:\n err:{} \n a(x): {:?} \n b(x): {:?} \n K: {}", fail, code.get_coef_a(), code.coef_b(), code.k());
    //  }
    //find_configs_5(81,18);
    //find_configs_3(81,18);

    //let v1 = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5];
    //let v2 = vec![2,3,4,5,1,3,4,5,1,2,4,5,1,2,3,5,1,2,3,4];

    //let length = v1.len();

    // let coef_a = vec![0,1,3];

    // let a = ParityCheckMatrix::circulant_right(&coef_a,60);

   // println!("Rank of a:{}",a.rank());

    // let (v1,v2) = a.to_edge_list();

    // let cr_nb = cr(&v1,&v2);

    // for i in 0..v1.len() {
    //     println!("({},{})",v1[i],v2[i]);
    // }

    // println!("Cr:{}",cr_nb);
    

    // let (v3,v4) = poly_to_edgelist(&coef_a,13);
    
    // let n = 9;
    // let indices = vec![0,6,7];

    // let matrix = ParityCheckMatrix::circulant_right_better(&indices, n);
    // let k = n-matrix.rank();
    // println!("k:{}",k);

    // let decoder_builder = EDBuilder::new(0.49);
    // let decoder = decoder_builder.from_code(matrix);
    // let simulator = ClassicalSimulator::new(decoder);
    // let err_rate = simulator.simulate_until_failures_are_found(5, 5).failure_rate();
    // println!("err rate:{}",err_rate);

    // let (v1,v2) = poly_to_edgelist(&indices,n);
    // let cr_nb = cr(&v1,&v2);

    // println!("cr:{}",cr_nb);

    // error_and_cr_3_par(24);
    //(22..30).for_each(|n| find_max_k_3(n));
    
    // for n in 6..60 {
    //     let max = find_max_k_3(n);
    //     if max != 0 {
    //         let count= count_code_3(n, max);
    //         let count_all= count_all_codes_3(n);
    //         println!("n:{}, max k:{}, nb:{}, all nb:{}", n,max,count, count_all);
    //     } else {
    //         println!("n:{}, max k:0",n);
    //     }  
    // }

    // for n in 50..100 {
    //     let count = count_all_codes_3(n);
    //     println!("n:{}, nb:{}", n,count);
    // }

    // for i in 1..6 {
    //     let n = 5*i;
    //     let k = 2*i;
    //     let count = count_code_5(n, k);
    //     println!("n:{}, nb:{}", n,count);
    // }

    // for n in 6..50{
    //     let max = find_max_k_3(n);
    //     println!("n:{}, max k:{}",n, max); 
    // }

    //test_myfamily(15,2,100);
    //find_configs_5(127, 13);
 
    
    // let a = ParityCheckMatrix::circulant_right_better(&vec![0,54,55,76,116], 127);
    // let b = ParityCheckMatrix::circulant_right_better(&vec![0,62,68,93,120], 127);
    // println!("k:{}", a.rank());
    // println!("k:{}", b.rank());
    // println!("k:{}", a.gbc(&b).rank());
   
    //error_and_cr_5_par_k(60,16);
    //println!("{:?}",ParityCheckMatrix::rand_ldpc(10, 5));
    //println!("{:?}",ParityCheckMatrix::rand_ldpc(10, 5));
   //error_and_cr_5_par_random(25,30);

//    (0..10).for_each( |_| {

//        let matrix = ParityCheckMatrix::rand_ldpc(25, 30, 5);
//        (0..100).into_par_iter().for_each(|_| {

//            let (v1,v2) = matrix.to_edge_list();
//            let cr = cr(&v1,&v2);

//            println!("{}",cr);

//        })

 
//test_family_5_wmul(15,2, 500);

for n in 3..30 {
    let mgg = fill_mgg(n);
    let n: usize = n.try_into().unwrap();
    let pcm = mgg_to_sparse(mgg, n);
    let r = pcm.rank();

    println!("n:{}, N:{}, k:{}",n, n*n, n*n-r);
}


Ok(())

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