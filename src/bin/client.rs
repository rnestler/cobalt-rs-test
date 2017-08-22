extern crate cobalt;

use cobalt::{
    BinaryRateLimiter, Client, Config, NoopPacketModifier, MessageKind, UdpSocket
};

fn main() {

    // Create a new client that communicates over a udp socket
    let mut client = Client::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default());

    // Initiate a connection to the server
    client.connect("127.0.0.1:1234").expect("Failed to bind to socket");

    loop {

        // Fetch connection events
        while let Ok(event) = client.receive() {
            println!("{:?}", event);
            // Handle events (e.g. Connection, Messages, etc.)
        }

        // Schedule a message to the send to the server
        if let Ok(connection) = client.connection() {
            connection.send(MessageKind::Instant, b"Ping".to_vec());
        }

        // Send all outgoing messages.
        //
        // Also auto delay the current thread to achieve the configured tick rate.
        client.send(true).unwrap();

    }

    // Disconnect the client, closing its connection and unbinding its socket
    //client.disconnect().unwrap();
}
