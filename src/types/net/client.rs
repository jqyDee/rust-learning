use std::{fmt::Display, net::SocketAddr};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast,
};

use super::Message;

pub struct Client {
    pub id: u64,
    pub name: String,
    pub sock_addr: SocketAddr,

    tx: broadcast::Sender<Message>,
    stream: TcpStream,
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: [id: {}, name: {}, ip: {}]",
            self.id, self.name, self.sock_addr
        )
    }
}

impl Client {
    pub fn from(
        id: u64,
        name: String,
        sock_addr: SocketAddr,
        tx: broadcast::Sender<Message>,
        stream: TcpStream,
    ) -> Self {
        Self {
            id,
            name,
            sock_addr,
            tx,
            stream,
        }
    }

    pub async fn conn_loop(self) {
        let client_str = self.to_string();

        println!("{} connected!", client_str);

        let mut rx = self.tx.subscribe();
        let (read, mut write) = self.stream.into_split();

        let mut lines = BufReader::new(read).lines();

        loop {
            tokio::select! {
                // broadcasting TO others
                res = lines.next_line() => {
                    match res {
                        Ok(Some(line)) => {
                            println!("{client_str}: {line}");
                            self.tx.send(Message::from(self.id, line)).ok();
                        },
                        _ => break,
                    }
                },

                // receive FROM others
                res = rx.recv() => {
                    match res {
                        Ok(msg) if msg.sender_id != self.id => {
                            write.write_all(format!("{client_str}: {msg}\n").as_bytes()).await.ok();
                        },
                        Ok(_) => {},
                        _ => break,

                    }
                }
            }
        }

        self.tx
            .send(Message::from(
                self.id,
                format!("{} disconnected", client_str),
            ).set_is_disconnect_msg())
            .ok();

        println!("{} disconnected!", client_str);
    }
}
