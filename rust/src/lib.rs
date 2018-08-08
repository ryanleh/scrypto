extern crate rpassword;
extern crate ring;
pub mod crypto;
pub mod file_handler;
use file_handler::FileHandler;
use crypto::{ Crypto };
use std::fmt;
use std::io;

pub enum Operation {
    DECRYPT,
    ENCRYPT,
}

#[derive(Debug)]
pub enum ScryptoError {
    Password,
    Integrity,
    Runtime,
    IO(io::Error),
}

impl From<ring::error::Unspecified> for ScryptoError {
    fn from(_: ring::error::Unspecified) -> Self { ScryptoError::Runtime }
}

impl From<io::Error> for ScryptoError {
    fn from(err: io::Error) -> Self { ScryptoError::IO(err)}
}

impl fmt::Display for ScryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScryptoError::Password => write!(f, "Incorrect password or file tampered with"),
            ScryptoError::Integrity => write!(f, "File has been tampered with"),
            ScryptoError::Runtime => write!(f, "Runtime error occured"),
            ScryptoError::IO(ref err) => write!(f, "IO error occured: {}", err)
        }
    }
}

/// Handles file_handler and crypto operations for generating enc file
fn encryptor(filename: &str, password: &str, remove: bool) -> Result<(), ScryptoError>{
    let mut ciphertext: Vec<u8> = Vec::new();
    let crypto = Crypto::new(password, None, None)?;
    let file_handler = FileHandler::new(filename, &Operation::ENCRYPT, remove)?;
    crypto.aes_encrypt(file_handler.content(), &mut ciphertext, filename)?;

    let content = crypto.pack_enc(&ciphertext);

    // Invariant: ciphertext takes up entire length of ciphertext vector
    file_handler.create_enc(content)?;
    Ok(())
}

/// Handles file_handler and crypto operations for decrypting enc file
fn decryptor(filename: &str, password: &str, remove: bool) -> Result<(), ScryptoError> {
    let file_handler = FileHandler::new(filename, &Operation::DECRYPT, remove)?;
    let (filename, content) = file_handler.unpack_enc()?;
    let (mut ciphertext, crypto) = Crypto::unpack_enc(password, content)?;
    
    let plaintext: &[u8];
    plaintext = crypto.aes_decrypt(&mut ciphertext, filename)?;
    file_handler.create_orig(plaintext, filename)?;
    Ok(())
}

pub fn run(operation: &Operation, remove: bool, filenames: Vec<&str>) {
    let password = rpassword::prompt_password_stdout("Password: ").unwrap();
    for filename in filenames.iter() {
        match operation {
            Operation::DECRYPT => {
                decryptor(filename, &password, remove).unwrap_or_else(|err| {
                    println!("Decrypting {} failed: {}", filename, err);
                });
            },
            Operation::ENCRYPT => {
                encryptor(filename, &password, remove).unwrap_or_else(|err| {
                    println!("Encrypting {} failed: {}", filename, err);
                });
            }
        };
    }
}
