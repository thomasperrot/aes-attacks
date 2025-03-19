use super::attack::DeltaSet;
use crate::aes::transform_state;
use crate::utils::transform::{plain_to_square, square_to_plain};
use crate::utils::types::Block;
use hex;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::str::from_utf8;

pub trait Oracle {
    fn encrypt(&mut self, delta_set: &mut DeltaSet) -> ();
}

pub struct LocalOracle {
    pub key: Block,
}

impl Oracle for LocalOracle {
    fn encrypt(&mut self, delta_set: &mut DeltaSet) {
        for mut ds in delta_set.iter_mut() {
            transform_state(&mut ds, &self.key, 4);
        }
    }
}

pub struct InvalidOracle {}

impl Oracle for InvalidOracle {
    fn encrypt(&mut self, _: &mut DeltaSet) -> () {}
}

pub struct RootMeOracle<'a> {
    pub stream: &'a mut TcpStream,
}

impl Oracle for RootMeOracle<'_> {
    fn encrypt(&mut self, delta_set: &mut DeltaSet) -> () {
        for ds in delta_set.iter_mut() {
            let plain = hex::encode(square_to_plain(ds));
            let request = format!("e {plain}\n");
            let mut response = [0u8; 33];
            println!("Sending {request}");
            if let Err(err) = self.stream.write(request.as_bytes()) {
                println!("Error when sending request: {err}");
                process::exit(-1)
            }
            if let Err(err) = self.stream.read(&mut response) {
                println!("Error when receiving response: {err}");
                process::exit(-1)
            }
            println!("Received {}", from_utf8(&response).unwrap());
            let encrypted_state =
                plain_to_square(&hex::decode(&response[0..32]).unwrap().try_into().unwrap());
            *ds = encrypted_state;
        }
    }
}

impl RootMeOracle<'_> {
    pub fn setup(&mut self) {
        let mut buf = [0; 256];
        self.stream.read(&mut buf).unwrap();
        print!("{}", from_utf8(&buf).expect("Invalid UTF8 sequence"));
        self.stream.read(&mut buf).unwrap();
        print!("{}", from_utf8(&buf).expect("Invalid UTF8 sequence"));
    }

    pub fn check_key(&mut self, key: &Block) -> bool {
        let request = format!("c {}\n", hex::encode(key));
        if let Err(err) = self.stream.write(request.as_bytes()) {
            println!("Error when sending request: {err}");
            process::exit(-1)
        }
        let mut response = [0u8; 100];
        if let Err(err) = self.stream.read(&mut response) {
            println!("Error when receiving response: {err}");
            process::exit(-1)
        }
        println!("Received {}", from_utf8(&response).unwrap());
        true
    }
}
