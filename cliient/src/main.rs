use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;

fn main() {
    println!("введите ip сервера в формате ");
    let mut servip = String::new();
    io::stdin().read_line(&mut servip).expect("bad");
    loop {
        let mut stream = TcpStream::connect("{}:4444", servip).unwrap();
        let mut unput = String::new();
        println!("введите текст");
        io::stdin().read_line(&mut unput).expect("bad");
        
        if unput.trim() == "exit" {
            break;
        }

        let msg = format!("{}\n", unput);
        stream.write_all(msg.as_bytes()).unwrap();

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        println!("ответ {}", String::from_utf8_lossy(&buf[..n]));
    }
}
