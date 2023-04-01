use serde::{Deserialize, Serialize};
use cmac::{Cmac, Mac};
use aes::Aes128;

// sv2 prefix for NTAG 424 is: 3cc300010080
const SV2_PREFIX: [u8; 6] = [0x3c, 0xc3, 0x00, 0x01, 0x00, 0x80];

pub fn build_sv2_message(uid: [u8; 7], count_lsb: [u8; 3]) -> [u8; 16] {
    let mut message: [u8; 16] = [0; 16];

    // set the message prefix
    for i in 0..6 {
        message[i] = SV2_PREFIX[i];
    }

    // append the uid
    for i in 0..7 {
        message[i + 6] = uid[i];
    }

    // append the count
    for i in 0..3 {
        message[i + 13] = count_lsb[i];
    }

    return message;
}

pub fn mac_message(key: [u8; 16], message: Vec<u8>) -> [u8; 16] {
    let mut mac = match Cmac::<Aes128>::new_from_slice(key.as_slice()) {
        Ok(m) => m,
        Err(_) => panic!("cmac creation error"),
    };
    mac.update(message.as_slice());
    let output: [u8; 16] = mac.finalize().into_bytes().as_slice().try_into().unwrap();

    return output;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inputs {
    pub count: [u8; 3],
    pub uid: [u8; 7],
    pub sun: [u8; 8],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub count: [u8; 3],
    pub uid: [u8; 7],
}

