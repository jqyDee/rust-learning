use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::broadcast;

use super::{CiArgs, ClientHandle, Message, client::Client};

const BROADCAST_CAP: usize = 32;

pub struct Server {
    pub sock_addr: SocketAddr,
    pub tx: broadcast::Sender<Message>,
    clients: HashMap<u64, ClientHandle>,
    pub next_id: u64,
}

impl Server {
    pub fn from_args(args: &CiArgs) -> Self {
        let (tx, _) = broadcast::channel::<Message>(BROADCAST_CAP);

        Self {
            sock_addr: SocketAddr::from((args.ip, args.port)),
            tx: tx,
            clients: HashMap::new(),
            next_id: 0
        }
    }

    pub async fn register_client(&mut self, client: Client) {
        let handle = ClientHandle::from(client.id, client.name.clone(), client.sock_addr);

        self.clients.insert(handle.id, handle);

        tokio::spawn(async move {
            client.conn_loop().await;
        });

        self.next_id += 1;
    }
}
