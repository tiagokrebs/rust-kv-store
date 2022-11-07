#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

// this funcintion handles  single client
fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);
    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data);
        if bytes_read.unwrap() == 0 {
            return Ok(());
        }
        let input: Point3d = serde_json::from_slice(&data)?;
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        write!(stream.get_mut(), "{}", f64::from(value).sqrt())?;
        write!(stream.get_mut(), "{}", String::from("\n"))?;
    }
}

fn main() {
    let _args: Vec<_> = env::args().collect();

    // server
    let listener = TcpListener::bind("0.0.0.0:6180").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("Failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
