#![no_main]

use risc0_zkvm::guest::env;
use nfc_core::{Inputs, Outputs, build_sv2_message, mac_message};

risc0_zkvm::guest::entry!(main);


static UIDS: &'static [[u8; 7]] = &[
    [0x04, 0xDE, 0x5F, 0x1E, 0xAC, 0xC0, 0x40],
    [0x04, 0x8F, 0x6A, 0x2A, 0xAA, 0x61, 0x80],
    [0x04, 0x81, 0x6A, 0x2A, 0xAA, 0x61, 0x80],
];

static KEYS: &'static [[u8; 16]] = &[
    [0x00; 16],
    [0x00; 16],
    [0x38, 0x78, 0x2F, 0x41, 0x3F, 0x44, 0x2A, 0x47, 0x2D, 0x4B, 0x61, 0x50, 0x64, 0x53, 0x67, 0x56],
];

pub fn main() {
    let inputs: Inputs = env::read();
    let id_index = UIDS.iter().position(|&u| u == inputs.uid).unwrap();
    let key = *KEYS.get(id_index).unwrap();

    let sv2 = build_sv2_message(inputs.uid, inputs.count);
    let macd_sv2 = mac_message(key, sv2.to_vec());

    let full_sun = mac_message(macd_sv2, Vec::new());
    let truncated_sun: [u8; 8] = [
        full_sun[1],
        full_sun[3],
        full_sun[5],
        full_sun[7],
        full_sun[9],
        full_sun[11],
        full_sun[13],
        full_sun[15],
    ];

    if truncated_sun != inputs.sun {
        panic!();
    }

    let out = Outputs {
        count: inputs.count,
        uid: inputs.uid,
    };
    env::commit(&out);
}
