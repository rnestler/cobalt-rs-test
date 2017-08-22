#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Command {
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Token(u32);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientMessage {
    Hello,
    Reconnect(Token),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerMessage {
    Hello(Token),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
