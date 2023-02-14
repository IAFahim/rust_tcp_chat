use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9111").await.unwrap();
    let (sender, _) = broadcast::channel::<(String,SocketAddr)>(10);
    loop {
        let (mut tcp_steam, addr) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let mut receiver = sender.subscribe();
        tokio::spawn(async move {
            let (reader_half, mut write_half) = tcp_steam.split();
            let mut buffer_reader = BufReader::new(reader_half);
            let mut line = String::new();
            loop {
                tokio::select! {
                    n = buffer_reader.read_line(&mut line) => {
                        if n.unwrap() == 0 {
                            break;
                        }
                        let _ = sender.send((line.clone(), addr));
                        line.clear();
                    }

                    msg=receiver.recv()=>{
                        let (msg, other_addr)=msg.unwrap();
                        if other_addr!=addr {
                            let _ = write_half.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }

            }
        });
    }
}

