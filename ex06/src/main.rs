use std::{
    env,
    fs::{self},
    error::{Error},
};
use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce
};

fn encrypter(input: &str, output: &str, arg_key: &[u8], arg_iv: &[u8]) -> Result<(), Box<dyn Error>> {
    
    let key = GenericArray::clone_from_slice(arg_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(arg_iv);

    let input_file = fs::read(input)?;

    let encrypted_tmp = cipher.encrypt(nonce.into(), input_file.as_ref());
    let encrypted;
        match encrypted_tmp {
            Ok(encrypted_tmp) => encrypted = encrypted_tmp,
            Err(_err) => return Err("Encryption error".into()),
        };

    fs::write(&output, encrypted)?;
    Ok(())
}

fn decrypter(input: &str, output: &str, key: &[u8], iv: &[u8]) -> Result<(), Box<dyn Error>> {

    let the_key = GenericArray::clone_from_slice(key);
    let cipher = Aes256Gcm::new(&the_key);
    let the_nonce = Nonce::from_slice(iv);
    let input_file = fs::read(input)?;

    let decrypted_tmp = cipher
        .decrypt(the_nonce.into(), input_file.as_ref());
    
    let decrypted;
    match decrypted_tmp {
        Ok(decrypted_tmp) => decrypted = decrypted_tmp,
        Err(_err) => return Err("Decryption error".into()),
    };

    fs::write(&output, &decrypted)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: cargo run [-enc/-dec] [input file] [output file] [aes_key] [iv]");
        return Ok(());
    }

    if args[1] == "-enc" {
        if args[4].len() != 32 || args[5].len() != 12 {
            println!("Key or IV not correct length! Please enter 32-bit key and 12-bit iv");
            return Ok(());
        }
        let ret = encrypter(&args[2], &args[3], &args[4].as_bytes(), &args[5].as_bytes());
        match ret {
            Ok(_ret) => return Ok(()),
            Err(_err) => return Err(_err.into()),
        };
    }
    else if args[1] == "-dec" {
        if args[4].len() != 32 || args[5].len() != 12 {
            println!("Key or IV not correct length! Please enter 32-bit key and 12-bit iv");
            return Ok(());
        }
        let ret = decrypter(&args[2], &args[3], &args[4].as_bytes(), &args[5].as_bytes());
        match ret {
            Ok(_ret) => return Ok(()),
            Err(_err) => return Err(_err.into()),
        };
    }
    else {
        println!("Usage: cargo run [-enc/-dec] [input file] [output file] [aes_key] [iv]");
        return Ok(());
    }
}