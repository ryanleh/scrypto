extern crate scrypto;
extern crate clap;
use clap::{Arg, App};
use scrypto::{Operation, run};

fn main() {
    let matches = App::new("scrypto")
        .author("Ryan Lehmkuhl <ryanleh.ob@gmail.com>")
        .version("1.0")
        .about("Encrypt files using 128-bit AES-GCM")
        .arg(Arg::with_name("filenames")
             .help("Files to encrypt/decrypt (Default is encrypt)")
             .required(true)
             .multiple(true))
        .arg(Arg::with_name("decrypt")
             .help("Sets mode to decrypt")
             .short("d")
             .long("decrypt"))
        .arg(Arg::with_name("remove")
             .help("Remove original file")
             .short("r")
             .long("remove"))
        .get_matches();

    let operation;
    if matches.is_present("decrypt") {
        operation = Operation::DECRYPT;
    } else {
        operation = Operation::ENCRYPT;
    }

    let mut remove = false;
    if matches.is_present("remove") {
        remove = true;
    }

    let filenames: Vec<&str> = matches.values_of("filenames").unwrap().collect();
    run(&operation, remove, filenames);
}