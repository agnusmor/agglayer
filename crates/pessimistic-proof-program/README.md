## Generate ELF file
cd pessimistic-proof-sp1/pessimistic-proof-program
cargo prove build --output-directory ./elf

## Generate Proof
cd pessimistic-proof-sp1/pessimistic-proof-prove
cargo run --release --bin ppprove-sp1