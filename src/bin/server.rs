extern crate cobalt;
extern crate cobalt_test;

use std::collections::HashMap;

use cobalt_test::Token;

use cobalt::{
    ConnectionID, BinaryRateLimiter, Server, Config, NoopPacketModifier, MessageKind, UdpSocket
};

/*
enum ClientState {
    Connected,
    Disconnected,
}
*/

struct Client {
    token: Token,
}

struct GameServer {
    connected_clients: HashMap<ConnectionID, Client>,
    disconnected_clients: Vec<Client>,
}

fn main() {
    // Create a new server that communicates over a udp socket
    let mut server = Server::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default());

    // Make the server listen on port `1234` on all interfaces.
    server.listen("0.0.0.0:1234").expect("Failed to bind to socket.");

    loop {

        // Accept incoming connections and fetch their events
        while let Ok(event) = server.accept_receive() {
            println!("{:?}", event);
            // Handle events (e.g. Connection, Messages, etc.)
        }

        // Send a message to all connected clients
        for (_, conn) in server.connections() {
            conn.send(MessageKind::Instant, b"Ping".to_vec());
        }

        // Send all outgoing messages.
        //
        // Also auto delay the current thread to achieve the configured tick rate.
        server.send(true).unwrap();

    }

    // Shutdown the server (freeing its socket and closing all its connections)
    //server.shutdown().unwrap();
}

