use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::str::from_utf8;
use crate::aes::transform_state;
use crate::utils::transform::{plain_to_square, square_to_plain};
use super::attack::DeltaSet;
use hex;

pub fn encrypt_delta_set(delta_set: &mut DeltaSet) {
    let key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    for mut ds in delta_set.iter_mut() {
        transform_state(&mut ds, &key);
    }
}


pub fn root_me_encrypt_delta_sets(&mut stream: TcpStream, delta_set: &mut DeltaSet) {
    let mut buf = [0; 256];
    stream.read(&mut buf).unwrap();
    print!("{}", from_utf8(&buf).expect("Invalid UTF8 sequence"));
    stream.read(&mut buf).unwrap();
    print!("{}", from_utf8(&buf).expect("Invalid UTF8 sequence"));
    for ds in delta_set.iter_mut() {
        let plain = hex::encode(square_to_plain(ds));
        let request = format!("e {plain}\n");
        let mut response = [0u8;33];
        println!("Sending {request}");
        if let Err(err) = stream.write(request.as_bytes()) {
            println!("Error when sending request: {err}");
            process::exit(-1)
        }
        if let Err(err) = stream.read(&mut response) {
            println!("Error when receiving response: {err}");
            process::exit(-1)
        }
        println!("Received {}", from_utf8(&response).unwrap());
        let encrypted_state = plain_to_square(&hex::decode(&response[0..32]).unwrap().try_into().unwrap());
        *ds = encrypted_state;
    }
}
