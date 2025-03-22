mod aes;
mod differential_fault_analysis;
mod square_attack;
mod utils;

use crate::square_attack::attack::crack_key;
use crate::square_attack::oracles::{LocalOracle, RootMeOracle};
use crate::utils::types::Block;
pub use differential_fault_analysis::attack as dfa_attack;
use std::net::TcpStream;
use std::process;
use std::time::SystemTime;

/// Benchmark the aes::encrypt function
///
/// Encrypting 1M plain texts currently takes 300ms using Rust, and 11min using Python. Which means
/// that Python in 2000 times slower than Rust for this specific case.
pub fn benchmark() {
    let key: Block = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let plain_text: Block = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let t_0 = SystemTime::now();
    for _ in 0..1_000_000 {
        aes::encrypt(&plain_text, &key);
    }
    let duration = t_0.elapsed().unwrap();
    let duration = duration.as_millis();
    println!("[+] Took {duration}ms");
}

pub fn crack_local() {
    let key: Block = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut oracle = LocalOracle { key };
    match crack_key(&mut oracle) {
        Some(cracked_key) => println!("[+] Found key {}", hex::encode(cracked_key)),
        None => println!("[-] Could not find key"),
    }
}

pub fn crack_root_me() {
    let mut stream =
        TcpStream::connect(("challenge01.root-me.org", 51039)).unwrap_or_else(|error| {
            println!("ERROR: {}", error.to_string());
            process::exit(-1)
        });
    let mut oracle = RootMeOracle {
        stream: &mut stream,
    };
    oracle.setup();
    let cracked_key = crack_key(&mut oracle).unwrap_or_else(|| {
        println!("[-] Could not find key");
        process::exit(-1);
    });
    println!("[+] Found key {}", hex::encode(cracked_key));
    oracle.check_key(&cracked_key);
}

pub fn test_dfa_attack() {
    let normal_cipher_text = [
        129, 214, 205, 195, 189, 22, 251, 141, 114, 185, 187, 136, 129, 139, 91, 233,
    ];
    let faulty_cipher_text = [
        239, 249, 53, 8, 99, 1, 135, 184, 211, 73, 78, 139, 112, 230, 136, 126,
    ];
    let t_0 = SystemTime::now();
    let keys = dfa_attack(&normal_cipher_text, &faulty_cipher_text);
    for key in keys.iter() {
        println!("{}", hex::encode(key));
    }
    println!("Took {}s", t_0.elapsed().unwrap().as_secs())
}
