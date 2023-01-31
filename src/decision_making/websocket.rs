
use tungstenite::accept;
use crate::libs::data::DataStore;
use serde::{Serialize, Deserialize};
use tungstenite::{Message, Result};
use flume::{Sender, Receiver, unbounded};
use std::error::Error;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use super::commands_wrapper::CommandsWrapper;
#[derive(Clone, Deserialize)]
enum ClientToServer {

}

#[derive(Serialize)]
enum ServerToClient {
    Store(DataStore)
}

pub struct WebSocketTask {
    rx: Receiver<ClientToServer>,
    tx: Sender<ServerToClient>,
}

impl WebSocketTask {
    pub fn new() -> Self {
        let (t_tx, s_rx) = unbounded();
        let (s_tx, t_rx) = unbounded();

        let mut server = WebSocketServer { rx: s_rx, tx: s_tx };
        tokio::spawn(async move { server.run().await });
        Self {
            rx: t_rx,
            tx: t_tx
        }
    }

    pub fn step(
        &mut self,
        commands_wrapper: &mut CommandsWrapper,
        store: &DataStore,
    ) -> Result<(), String> {
        self.tx.send(ServerToClient::Store(store.clone())).unwrap();
        Ok(())
    }
}


struct WebSocketServer {
    rx: Receiver<ServerToClient>,
    tx: Sender<ClientToServer>
}

impl WebSocketServer {
    async fn handle_connection(
        raw_stream: TcpStream,
        addr: SocketAddr,
        rx: Receiver<ServerToClient>,
        tx: Sender<ClientToServer>
    ) {
        // TODO: Clean up and handle errors
        println!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");
        println!("WebSocket connection established: {}", addr);
    
    
        let (outgoing, incoming) = ws_stream.split();
        let incoming_fut = incoming.filter_map(|x| async move { 
            if let Ok(Message::Text(m)) = x {
                if let Ok(req) = serde_json::from_str::<ClientToServer>(&m) {
                    return Some(req);
                }
            }

            None
        }).map(Ok).forward(tx.sink());
        let outgoing_fut = rx.stream()
        .map(|m| Message::text(serde_json::to_string(&m).unwrap()))
        .map(Ok)
        .forward(outgoing);
        let (incoming_res, outgoing_res) = tokio::join!(incoming_fut, outgoing_fut);
        println!("{} disconnected", &addr);
    }

    async fn run(&mut self) {
        let addr = "127.0.0.0:5001";
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(Self::handle_connection( stream, addr, self.rx.clone(), self.tx.clone()));
        }
    }
}