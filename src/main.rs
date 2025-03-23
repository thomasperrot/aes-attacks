use clap::Parser;
use aes_attacks::dfa_attack;

#[derive(Parser)]
struct Cli {
    normal_cipher_text: String,
    faulty_cipher_text: String,
    nb_threads: u8,
}
fn main() {
    let args = Cli::parse();
    let normal_cipher_text = hex::decode(&args.normal_cipher_text).unwrap().try_into().unwrap();
    let faulty_cipher_text = hex::decode(&args.faulty_cipher_text).unwrap().try_into().unwrap();
    let keys = dfa_attack(&normal_cipher_text, &faulty_cipher_text, args.nb_threads);
    for key in keys {
        println!("{}", hex::encode(key));
    }
}
