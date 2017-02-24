use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:45678")
        .unwrap();

    let stream = listener.accept().unwrap().0;
    read_request(stream);
}

fn read_request(mut stream: TcpStream){

    let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World ! ❤️";

    stream.write(response.as_bytes()).unwrap();
}
