#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::{env, str};

#[derive(Serialize, Deserialize, Debug)]
struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    let _args: Vec<_> = env::args().collect();

    // client
    let mut stream = TcpStream::connect("127.0.0.1:6180").expect("Could not connecto to server");
    println!("Provide a 3D point as three comma separated integers");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let parts: Vec<&str> = input.trim_matches('\n').split(',').collect();
        let point = Point3d {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        };

        stream
            .write_all(serde_json::to_string(&point).unwrap().as_bytes())
            .expect("Failed to weite to server");
        stream.write_all(b"\n").expect("Failed to write to server");
        let mut reader = BufReader::new(&stream);
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        let input = str::from_utf8(&buffer).expect("Courl not write buffer as string");
        if input == "" {
            eprintln!("Error response from server");
        }
        println!("Response from server {}", input);
    }
}
