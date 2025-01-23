use std::{fs::File, io::Read};
pub use pessimistic_proof::{LocalNetworkState, PessimisticProofOutput};

/// The ELF we want to execute inside the zkVM.
pub const PESSIMISTIC_PROOF_ELF: &[u8] =
    include_bytes!("../../pessimistic-proof-program/elf/pessimistic-proof-program");

pub fn main() {
    sp1_sdk::utils::setup_logger();

    let client = sp1_sdk::ProverClient::new();
    let mut stdin = sp1_sdk::SP1Stdin::new();

    let mut file = File::open("build/input.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    
    stdin.write_vec(buffer);

    let (pk, _) = client.setup(PESSIMISTIC_PROOF_ELF);

    let _ = client.prove(&pk, stdin).plonk().run().expect("failed to prove");

    // let output = PessimisticProofOutput::bincode_options()
    //     .deserialize(public_vals.as_slice())
    //     .expect("deser");
}
