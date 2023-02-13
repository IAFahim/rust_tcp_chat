use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9111").await.unwrap();
    let ( sender, _) = broadcast::channel::<String>(10);
    loop {
        let (mut tcp_steam, addr) = listener.accept().await.unwrap();
        let sender=sender.clone();
        let mut receiver=sender.subscribe();
        tokio::spawn(async move {
            let (read, mut write) = tcp_steam.split();
            let mut buffer_reader = BufReader::new(read);
            let mut line = String::new();
            loop {
                let size = buffer_reader.read_line(&mut line).await.unwrap();
                if size==0{
                   break;
                }

                sender.send(line.clone()).unwrap();
                let msg= receiver.recv().await.unwrap();
                write.write_all(msg.as_bytes()).await.unwrap();

                line.clear();
            }
        });
    }
}

