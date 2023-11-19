use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::io::Write;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    // Kết nối đến server
    let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    println!("Connected to server!");
    // let mut buf_reader = BufReader::new(&mut stream);

 
    // // Task để nhận và in tin nhắn từ server
    // tokio::spawn(async move {
    //     loop {
    //         let mut buffer = String::new();
    //         buf_reader.read_line(&mut buffer).await.unwrap();
    //     }
    // });

    // Đọc từ bàn phím và gửi tới server
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        reader.read_line(&mut line).await.unwrap();

        // Gửi tin nhắn đến server
        stream.write_all(line.as_bytes()).await.unwrap();

        line.clear();
    }
}