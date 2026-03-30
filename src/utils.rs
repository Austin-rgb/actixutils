use std::{env, fs};

pub fn get_private_key() -> String {
    let p = env::var("RSA_PRIVATE_KEY_PATH").expect("missing RSA_PRIVATE_KEY_PATH");
    fs::read_to_string(&p).expect(&("could not access".to_string() + &p))
}

pub fn get_public_key() -> String {
    let p = env::var("RSA_PUBLIC_KEY_PATH").expect("missing RSA_PUBLIC_KEY_PATH");
    fs::read_to_string(&p).expect(&("could not access".to_string() + &p))
}
