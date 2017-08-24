extern crate cobalt;
extern crate cobalt_test;

use std::collections::hash_map::{HashMap, Entry};
use std::ops::RangeFrom;

use cobalt_test::Token;

use cobalt::{
    ConnectionID, BinaryRateLimiter, Server, Config, NoopPacketModifier, MessageKind, UdpSocket, ServerEvent
};

enum ClientState {
    Connecting,
    Connected(Token),
}

struct Client {
    token: Token,
}

impl Client {
    pub fn new(token: Token) -> Client {
        Client { token: token }
    }
}

struct GameServer {
    server: Server<UdpSocket, BinaryRateLimiter, NoopPacketModifier>,
    connected_clients: HashMap<ConnectionID, Client>,
    disconnected_clients: Vec<Client>,
    client_token_generator: RangeFrom<u32>,
}

impl GameServer {
    pub fn new() -> GameServer {
        // Create a new server that communicates over a udp socket
        GameServer {
            server: Server::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default()),
            connected_clients: HashMap::<ConnectionID, Client>::new(),
            disconnected_clients: vec![],
            client_token_generator: 0..,
        }
    }

    pub fn listen(&mut self, interface: &str) -> Result<(), std::io::Error> {
        self.server.listen(interface)
    }

    fn handle_connect(&mut self, connection_id: ConnectionID) {
        //self.disconnected_clients.iter().find(connection_id);
        println!("Connect from {:?}", connection_id);
        match self.connected_clients.entry(connection_id) {
            Entry::Occupied(_) => {
                println!("alrady here...");
            },
            Entry::Vacant(entry) => {
                let token = self.client_token_generator.next().unwrap();
                entry.insert(Client::new(token.into()));
            },
        }
    }

    fn handle_disconnect(&mut self, connection_id: ConnectionID) {
        println!("Disconnect from {:?}", connection_id);
        self.connected_clients.remove(&connection_id);
        /*
        match self.connected_clients.entry(connection_id) {
            Entry::Occupied(entry) => {
                println!("alrady here...");
            },
            Entry::Vacant(entry) => {
                let token = self.client_token_generator.next().unwrap();
                entry.insert(Client::new(token.into()));
            },
        }
        */
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            // Accept incoming connections and fetch their events
            while let Ok(event) = self.server.accept_receive() {
                match event {
                    ServerEvent::Connection(connection_id) => {
                        self.handle_connect(connection_id);
                    },
                    ServerEvent::ConnectionLost(connection_id, _) |
                    ServerEvent::ConnectionClosed(connection_id, _) => {
                        self.handle_disconnect(connection_id);
                    },
                    _ => println!("{:?}", event),
                }
            }

            // Send a message to all connected clients
            for (_, conn) in self.server.connections() {
                conn.send(MessageKind::Instant, b"Ping".to_vec());
            }

            // Send all outgoing messages.
            //
            // Also auto delay the current thread to achieve the configured tick rate.
            self.server.send(true)?
        }
    }

    pub fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.server.shutdown()
    }
}

fn main() {
    let mut game_server = GameServer::new();
    game_server.listen("0.0.0.0:1234").expect("Failed to bind to socket.");

    game_server.run().unwrap();


    // Shutdown the server (freeing its socket and closing all its connections)
    game_server.shutdown().unwrap();
}

