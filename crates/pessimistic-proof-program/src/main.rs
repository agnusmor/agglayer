#![no_main]
sp1_zkvm::entrypoint!(main);

use bincode::Options;
use serde::{Deserialize, Serialize};
use pessimistic_proof::{
    generate_pessimistic_proof, 
    LocalNetworkState, 
    PessimisticProofOutput,
    multi_batch_header::MultiBatchHeader,
    local_exit_tree::hasher::Keccak256Hasher
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct PessimisticProofInput {
    pub state: LocalNetworkState,
    pub batch_header: MultiBatchHeader<Keccak256Hasher>,
}

pub fn main() {
    // Get the input slice from sp1
    let input = sp1_zkvm::io::read_vec();

    let pp_input: PessimisticProofInput = bincode::deserialize(&input).unwrap();

    let outputs = generate_pessimistic_proof(pp_input.state, &pp_input.batch_header).unwrap();

    let pp_inputs = PessimisticProofOutput::bincode_options()
         .serialize(&outputs)
         .unwrap();    

    // Set the output values in sp1
    sp1_zkvm::io::commit_slice(&pp_inputs);        
}