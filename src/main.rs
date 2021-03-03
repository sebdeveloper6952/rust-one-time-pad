use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // random number generator
    let mut rng = rand::thread_rng();

    let msgs = vec![
        "plain text message 1",
        "this is a super secret message that needs super encryption",
    ];

    // encrypt messages and write bytes to file
    for i in 0..msgs.len() {
        let bytes = msgs[i].as_bytes().to_vec();
        let mut pad = Vec::new();
        for _ in 0..msgs[i].len() {
            let rnd: u8 = rng.gen();
            pad.push(rnd);
        }
        let pad_s: Vec<String> = pad.iter().map(|a| a.to_string()).collect();
        let path = format!("pad_{}", i);
        let mut file = File::create(path).unwrap();
        writeln!(file, "{}", pad_s.join(","));
        let enc: Vec<u8> = bytes.iter().zip(pad.iter()).map(|(&a, &b)| a ^ b).collect();
        let enc_s: Vec<String> = enc.iter().map(|a| a.to_string()).collect();
        let path = format!("encrypted_{}", i);
        let mut file = File::create(path).unwrap();
        writeln!(file, "{}", enc_s.join(","));
    }

    // read encrypted messages and pads and decrypt
    for i in 2..4 {
        let path = format!("encrypted_{}", i);
        let pad_path = format!("pad_{}", i);
        let mut file = File::open(path).unwrap();
        let mut pad_file = File::open(pad_path).unwrap();
        let mut encrypted = String::new();
        let mut pad = String::new();
        file.read_to_string(&mut encrypted).unwrap();
        pad_file.read_to_string(&mut pad).unwrap();
        encrypted = str::replace(&encrypted, "\n", "");
        encrypted = str::replace(&encrypted, " ", "");
        pad = str::replace(&pad, "\n", "");
        pad = str::replace(&pad, " ", "");
        let encrypted_split: Vec<&str> = encrypted.split(',').collect();
        let pad_split: Vec<&str> = pad.split(',').collect();
        let enc_bytes: Vec<u8> = encrypted_split
            .iter()
            .map(|a| a.parse::<u8>().unwrap())
            .collect();
        let pad_bytes: Vec<u8> = pad_split.iter().map(|a| a.parse::<u8>().unwrap()).collect();
        let decrypted: Vec<u8> = enc_bytes
            .iter()
            .zip(pad_bytes.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();
        println!("desencriptando mensaje {:?} con pad {:?}", encrypted, pad);
        println!("{}", String::from_utf8(decrypted).unwrap());
    }
}
