use std::{env, fs};

use libsigners::{HS256Signer, RS256Signer, RS256Validator, Sign, Validate};

enum KeyType {
    HS256,
    RS256,
}

fn key_type() -> KeyType {
    let binding = env::var("KEY_TYPE").expect("KEY_TYPE not set");
    let key_type = binding.as_str();
    if key_type == "rs256" {
        KeyType::RS256
    } else {
        KeyType::HS256
    }
}

fn load_rs256_validator(aud: String) -> Box<dyn Validate> {
    let p = env::var("RSA_PUBLIC_KEY_PATH").expect("missing RSA_PUBLIC_KEY_PATH");
    let public_key = fs::read_to_string(&p).expect(&("could not access".to_string() + &p));

    Box::new(RS256Validator::new(public_key, aud))
}

fn load_rs256_signer(aud: String) -> Box<dyn Sign> {
    let p = env::var("RSA_PRIVATE_KEY_PATH").expect("missing RSA_PRIVATE_KEY_PATH");
    let public_key = fs::read_to_string(&p).expect(&("could not access".to_string() + &p));

    Box::new(RS256Signer::new(public_key, aud))
}

fn load_hs256_validator(aud: String) -> Box<dyn Validate> {
    Box::new(HS256Signer::new(aud))
}

fn load_hs256_signer(aud: String) -> Box<dyn Sign> {
    Box::new(HS256Signer::new(aud))
}

pub fn load_validator(aud: String) -> Box<dyn Validate> {
    let binding = env::var("KEY_TYPE").expect("KEY_TYPE not set");
    let key_type = binding.as_str();
    if key_type == "rs256" {
        load_hs256_validator(aud)
    } else {
        load_rs256_validator(aud)
    }
}

pub fn load_signer(aud: String) -> Box<dyn Sign> {
    let binding = env::var("KEY_TYPE").expect("KEY_TYPE not set");
    let key_type = binding.as_str();
    if key_type == "rs256" {
        load_hs256_signer(aud)
    } else {
        load_rs256_signer(aud)
    }
}
