use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1/r/n";

    // if buffer.starts_with(get)
    // {
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    //     stream.write(response.as_bytes()).unwrap();   // 对所有错误指令使用unwrap
    //     stream.flush().unwrap();
    // }else
    // {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!("{}{}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();

    // }

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 把上面代码重构一下
    let (status_line, filename) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else
    {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
