use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9111").await.unwrap();
    let (sender, _) = broadcast::channel::<String>(10);
    loop {
        let (mut tcp_steam, addr) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let mut receiver = sender.subscribe();
        tokio::spawn(async move {
            let (reader_Half, mut write_half) = tcp_steam.split();
            let mut buffer_reader = BufReader::new(reader_Half);
            let mut line = String::new();
            loop {
                tokio::select! {
                    n = buffer_reader.read_line(&mut line) => {
                        if n.unwrap() == 0 {
                            break;
                        }
                        let _ = sender.send(line.clone());
                        line.clear();
                    }

                    msg=receiver.recv()=>{
                        let msg: String=msg.unwrap();
                        let _ = write_half.write_all(msg.as_bytes()).await.unwrap();
                    }

                }
            }
        });
    }
}

