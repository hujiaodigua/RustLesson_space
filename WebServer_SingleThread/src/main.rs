use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() 
{
    // 监听传入的流并在接收到流时打印信息
    let listener = TcpListener::bind("192.168.0.188:9878").unwrap();

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        println!("Connection established!");

        // 读取请求
        handle_connection(stream);
    }

    println!("Connection established!");
}
