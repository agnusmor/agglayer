## Generate ELF file
cd pessimistic-proof-sp1/pessimistic-proof-program
cargo prove build --output-directory ./elf

## Generate Proof
cd pessimistic-proof-sp1/pessimistic-proof-prove
cargo run --release --bin ppprove-sp1

## Notes
- Public input size 196 bytes (Zisk = 49 x 4. Serialized using bincode)