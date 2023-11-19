use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{broadcast, Mutex},
    fs::File,
};
use std::sync::Arc;


#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening ...");

    let (tx, rx) = broadcast::channel(10);

    let history_file = Arc::new(Mutex::new(File::create("chat_history.txt").await.unwrap()));


    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let history_file = Arc::clone(&history_file);

        {
            let mut history_file = history_file.lock().await;
            history_file.write_all(format!("Accepted - {}\n", &addr).as_bytes()).await.unwrap();
            println!("Connected - {}", addr);
        }


        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            
            loop{
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            let msg_disconnected = format!("Disconnected {}", addr);
                            println!("{}", msg_disconnected);
                            let mut history_file = history_file.lock().await;
                            history_file.write_all(msg_disconnected.as_bytes()).await.unwrap();
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();

                        let msg: String = format!("{}: {}", addr.to_string(), line.to_string());
                        let mut history_file = history_file.lock().await;
                        history_file.write_all(msg.as_bytes()).await.unwrap();
                        
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            let msg: String = format!("{}: {}", other_addr.to_string(), msg.to_string());
                            write.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}