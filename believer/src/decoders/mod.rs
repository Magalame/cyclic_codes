use crate::ParityCheckMatrix;

pub trait Code<Cks> {
    fn get_checks(&self) -> Cks;
}


pub trait Decoder: Send + Sync {
    type Error;
    type Result: DecodingResult;
    fn copy_pcm(&self) -> ParityCheckMatrix;
    fn decode(&self, error: &Self::Error) -> Self::Result;
    fn decode_mut(&self, error: &Vec<usize>, work_pcm: &mut ParityCheckMatrix) -> Self::Result;
    fn random_error(&self) -> Self::Error;
    fn random_error_mut(&self, error: &mut Vec<usize>);
}
//    fn new(checks: &I, erasure_prob: f64);

pub trait DecodingResult: Send + Sync {
    fn succeed(&self) -> bool;

    fn failed(&self) -> bool {
        !self.succeed()
    }
}

pub trait DecoderBuilder<Cks, D> where
D: Decoder{

    fn from_code(&self,code: Cks) -> D;

}

pub mod belief_propagation;
pub use belief_propagation::{BPDecoder, BPResult};

pub mod erasure;
pub use erasure::{ErasureDecoder, ErasureResult, EDBuilder};

pub mod css_erasure;
pub use css_erasure::{CSSErasureDecoder, CSSErasureResult, CSSEDBuilder};