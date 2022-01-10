use std::borrow::Borrow;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8888").await.unwrap();
    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                //
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let message = result.unwrap();
                        writer.write_all(message.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
