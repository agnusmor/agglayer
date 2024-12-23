#![no_main]
ziskos::entrypoint!(main);

use bincode::Options;
use serde::{Deserialize, Serialize};
use pessimistic_proof::{
    generate_pessimistic_proof, 
    LocalNetworkState, 
    PessimisticProofOutput,
    multi_batch_header::MultiBatchHeader,
    local_exit_tree::hasher::Keccak256Hasher
};
use ziskos::{read_input, set_output};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct PessimisticProofInput {
    pub state: LocalNetworkState,
    pub batch_header: MultiBatchHeader<Keccak256Hasher>,
}

pub fn main() {
    // Get the input slice from ziskos
    let input  = read_input();

    let pp_input: PessimisticProofInput = bincode::deserialize(&input).unwrap();

    let outputs = generate_pessimistic_proof(pp_input.state, &pp_input.batch_header).unwrap();

    let pp_inputs = PessimisticProofOutput::bincode_options()
         .serialize(&outputs)
         .unwrap();    

    // Set the output values in ziskos
    for (index, chunk) in pp_inputs.chunks(4).enumerate() {
        let value = if chunk.len() == 4 {
            u32::from_le_bytes(chunk.try_into().unwrap())
        } else {
            // Handle cases where the chunk is not 4 bytes (e.g., if the Vec length is not divisible by 4)
            let mut padded = [0u8; 4];
            padded[..chunk.len()].copy_from_slice(chunk);
            u32::from_le_bytes(padded)
        };
        set_output(index, value);
    }        
}