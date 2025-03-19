use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::io::{BufRead, BufReader, Lines, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

pub struct Client {
    stream: TcpStream,
    pub(crate) host: Option<String>,

    pub address: SocketAddr,
    pub introduced: bool,
}

pub struct ClientPool {
    clients: HashMap<SocketAddr, Client>
}

pub enum ClientPoolError {
    CantResolvePeer
}

impl ClientPool {
    pub fn new() -> Self {
        ClientPool {
            clients: HashMap::new()
        }
    }

    pub fn get_or_create_client(&mut self, stream: TcpStream) -> Result<&mut Client, ClientPoolError> {
        if let Ok(address) = stream.peer_addr() {
            return Ok(&mut *self.clients.entry(address).or_insert(Client {
                stream,
                host: None,

                address,
                introduced: false,
            }));
        }
        Err(ClientPoolError::CantResolvePeer)
    }
}

impl Client {
    pub fn read(&self) -> Lines<BufReader<TcpStream>> {
        let reader = BufReader::new(self.stream.try_clone().unwrap());

        reader.lines()
    }

    pub fn disconnect(&self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn write<T: Into<String>>(&mut self, message: T) {
        self.stream.write(message.into().as_bytes()).unwrap();
    }

    pub fn writeln<T: Display>(&mut self, message: T) {
        let mut vec = Vec::new();
        vec.extend_from_slice(message.to_string().as_bytes());
        vec.push(b'\n');
        self.stream.write(vec.as_slice()).unwrap();
    }

    pub fn hostname(&mut self, hostname: String) {
        self.host = Some(hostname);
    }
}