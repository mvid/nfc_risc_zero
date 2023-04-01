use methods::{AES_CMAC_ELF, AES_CMAC_ID};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use nfc_core::{Inputs, Outputs};


fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(AES_CMAC_ELF).expect("Prover should be constructed from valid ELF binary");

    // let key: [u8; 16] = [0x00; 16];
    // let uid: [u8; 7] = [0x04, 0xDE, 0x5F, 0x1E, 0xAC, 0xC0, 0x40];
    // let count : [u8; 3] = [0x3D, 0x00, 0x00]; // count is LSB encoded
    // let expected_sun: [u8; 8] = [0x94, 0xEE, 0xD9, 0xEE, 0x65, 0x33, 0x70, 0x86];

    // let key: [0x38, 0x78, 0x2F, 0x41, 0x3F, 0x44, 0x2A, 0x47, 0x2D, 0x4B, 0x61, 0x50, 0x64, 0x53, 0x67, 0x56]
    let uid: [u8; 7] =  [0x04, 0x81, 0x6A, 0x2A, 0xAA, 0x61, 0x80];
    let count: [u8; 3] = [0x02, 0x00, 0x00];
    let expected_sun: [u8; 8] = [0x49, 0x90, 0x53, 0x7A, 0x97, 0x89, 0xB3, 0xE6];
    let inputs = Inputs{
        count,
        uid,
        sun: expected_sun,
    };
    prover.add_input_u32_slice(&to_vec(&inputs).unwrap());

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    let journal = &receipt.journal;
    let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

    println!("journal: {:?}", outputs);
    let expected_outputs = Outputs {
        count,
        uid,
    };
    println!("expected journal: {:?}", expected_outputs);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&AES_CMAC_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
}
