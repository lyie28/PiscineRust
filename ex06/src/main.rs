use std::{
    env,
    fs::{self}
};
use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce
};
use anyhow::anyhow;

fn encrypter(input: &str, output: &str, arg_key: &[u8], arg_iv: &[u8]) -> Result<(), anyhow::Error> {
    
    //32 key + 12 nonce
    let key = GenericArray::clone_from_slice(arg_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(arg_iv); // 96-bits; unique per message

    let input_file = fs::read(input)?;

    //get rid of anyhow :(!
    let encrypted = cipher
        .encrypt(nonce.into(), input_file.as_ref())
        .map_err(|err| anyhow!("Encrypting: {}", err))?;
    fs::write(&output, encrypted)?;
    Ok(())
}

fn decrypter(input: &str, output: &str, key: &[u8], iv: &[u8]) -> Result<(), anyhow::Error> {

    let the_key = GenericArray::clone_from_slice(key);
    let cipher = Aes256Gcm::new(&the_key);
    let the_nonce = Nonce::from_slice(iv); // 96-bits; unique per message
    let input_file = fs::read(input)?;

    //get rid of anyhow!
    let decrypted = cipher
        .decrypt(the_nonce.into(), input_file.as_ref())
        .map_err(|err| anyhow!("Encrypting: {}", err))?;

    fs::write(&output, &decrypted)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: cargo run [-enc/-dec] [input file] [output file] [aes_key] [iv]");
        return;
    }

    if args[1] == "-enc" {
        if args[4].len() != 32 || args[5].len() != 12 {
            println!("Key or IV not correct length! Please enter 32-bit key and 12-bit iv");
            return;
        }
        let _ret = encrypter(&args[2], &args[3], &args[4].as_bytes(), &args[5].as_bytes());
    }
    else if args[1] == "-dec" {
        if args[4].len() != 32 || args[5].len() != 12 {
            println!("Key or IV not correct length! Please enter 32-bit key and 12-bit iv");
            return;
        }
        let _ret = decrypter(&args[2], &args[3], &args[4].as_bytes(), &args[5].as_bytes());
    }
    else {
        println!("Usage: cargo run [-enc/-dec] [input file] [output file] [aes_key] [iv]");
        return;
    }
}