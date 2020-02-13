use super::CodeGenerator;
use believer::{ParityCheckMatrix,CSSSimulator};
use believer::{Decoder, Code};
use rand::distributions::{Distribution, Uniform};


pub struct GBC {
    n_bits: usize,
    coef_a: Vec <usize>,
    coef_b: Vec <usize>,
    hx: ParityCheckMatrix,
    hz: ParityCheckMatrix,
    k: usize,
    g: Option<Vec <usize>>,

}

impl GBC{
    pub fn get_coef_a(&self) -> &Vec <usize> {
        &self.coef_a
    }
    pub fn coef_b(&self) -> &Vec <usize>{
        &self.coef_b
    }
    pub fn hx(&self) -> &ParityCheckMatrix {
        &self.hx
    }
    pub fn hz(&self) -> &ParityCheckMatrix{
        &self.hz
    }
    pub fn k(&self) -> usize {
        self.k
    }
}

impl Code<(ParityCheckMatrix,ParityCheckMatrix)> for GBC {
    fn get_checks(&self) -> (ParityCheckMatrix, ParityCheckMatrix) {
        (self.hx.clone(), self.hz.clone())
    }

    // fn get_n(&self) -> usize {
    //     self.n_bits
    // }

    // fn get_w(&self) -> usize {
    //     self.coef_a.len() + self.coef_b.len()
    // }

    // fn get_k(&self) -> usize {
    //     self.k
    // }
}

pub struct GBCGenerator {
    n_bits: usize,
    min_weight: usize,
    max_weight: usize,
    k: Option<usize>
}

impl GBCGenerator {
    pub fn new(n_bits: usize,
    min_weight: usize,
    max_weight: usize,
    k: Option<usize>) -> GBCGenerator {
        Self {
            n_bits,
            min_weight,
            max_weight,
            k
        }
    }
}

impl<D> CodeGenerator<GBC,(ParityCheckMatrix,ParityCheckMatrix), D>  for GBCGenerator 
where D: Decoder{
    
    type SimulatorType = CSSSimulator<D>;

    fn generate(&self) -> GBC {

        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..self.n_bits);

        let mut coef_a: Vec <usize> = Vec::with_capacity(self.max_weight);
        let mut coef_b: Vec <usize> = Vec::with_capacity(self.max_weight);

        let (hx,hz,k) = loop {

            /*
            for _ in 0..self.min_weight/2 { 
                let throw = die.sample(&mut rng);
                let throw2 = die.sample(&mut rng);
            
                coef_a.push(throw);
                coef_b.push(throw2);
            
            }

            coef_a.sort();
            coef_b.sort();
            */

            coef_a = vec![0,15,20,28,66];
            coef_b = vec![0,58,59,100,121];

           // coef_a = vec![0,1,2,9,51];
            //coef_b = vec![0,58,59,100,121];
            // coef_a = vec![0,89,94,97,105];
            // coef_b = vec![0,106,118,123,126];

            let a = ParityCheckMatrix::circulant_right(&coef_a,self.n_bits);
            let b = ParityCheckMatrix::circulant_right(&coef_b,self.n_bits);

            println!("Rank of a:{}",a.rank());
            println!("Rank of b:{}",b.rank());


            let hx = a.right_concat(&b);
            let hz = b.transpose().right_concat(&a.transpose());

            let h = hx.diag_concat(&hz);
            let h_rank = h.rank();
            let k = 2*self.n_bits - h_rank;

            println!("k:{}",k);

            if h_rank < 2*self.n_bits { // we want to encode at least one qubit
                break (hx,hz,k)
            }

            println!("Failed:\n a(x): {:?} \n b(x): {:?} \n k = {}",coef_a,coef_b,k);
            
            coef_a.truncate(0);
            coef_b.truncate(0);
            break (hx,hz,k)

        };

        GBC{ n_bits: 2*self.n_bits,
             coef_a,
             coef_b,
             hx : hx,
             hz : hz,
             k,
             g: None }
        

    }

}