use std::net::TcpStream;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::io;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").unwrap();
    println!("сервер запущен на 4444");

    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

fn handle(mut stream: TcpStream) {
    let mut server = String::new();
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).unwrap();
    let msg = String::from_utf8_lossy(&buf[..n]);
    println!("получено: {msg}");
    println!("введите ответ");
    io::stdin().read_line(&mut server).expect("bad");
    let msg = format!("{}\n", server);
    stream.write_all(msg.as_bytes()).unwrap();
}
