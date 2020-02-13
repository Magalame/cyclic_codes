//! A belief propapagation decoder for classical and quantum sparse error correcting codes.

pub mod channel;
pub use channel::BinaryChannel;

pub mod decoders;
pub use decoders::{BPDecoder, BPResult, Decoder, DecoderBuilder, Code, DecodingResult, ErasureDecoder, EDBuilder, ErasureResult, CSSErasureDecoder, CSSEDBuilder, CSSErasureResult};

pub mod gf2;
pub use gf2::GF2;

pub mod parity_check_matrix;
pub use parity_check_matrix::{ParityCheckMatrix,add_checks,add_checks_mut};

pub mod simulation;
pub use simulation::{SimulationResult, Simulator,ClassicalSimulator,CSSSimulator};

mod sparse_matrix;
