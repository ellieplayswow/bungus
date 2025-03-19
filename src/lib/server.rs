use std::net::{Shutdown, TcpListener};
use std::thread;
use crate::lib::client::{Client, ClientPool};
use crate::lib::message::{ClientMessage, ServerMessage};

pub struct Server {
    host: String,
    port: u16,

    client_pool: ClientPool,
    listener: Option<TcpListener>,

    on_connect: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            host: String::from("127.0.0.1"),
            port: 25,

            client_pool: ClientPool::new(),
            listener: None,

            on_connect: None,
        }
    }

    pub fn port(mut self, port: u16) -> Server {
        self.port = port;
        self
    }

    pub fn host(mut self, host: String) -> Server {
        self.host = host;
        self
    }

    pub fn listen(mut self) {
        self.listener = Some(TcpListener::bind(format!("{}:{}", self.host, self.port)).expect("Failed to bind"));

        for stream in self.listener.unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let client = self.client_pool.get_or_create_client(stream);

                    if let Ok(client) = client {

                        if !client.introduced {
                            client.writeln(ServerMessage::ServiceReady);
                            client.introduced = true;
                        }

                        for line in client.read() {
                            if let Ok(line) = line {
                                Self::handle_message(client, ClientMessage::from(line));
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
        }
    }

    fn handle_message(client: &mut Client, message: ClientMessage) {
        match message {
            ClientMessage::Helo(hostname) => {
                println!("client {} identifying as {}", client.address, hostname);
                client.hostname(hostname.clone());
                client.writeln(ServerMessage::Okay(Some(format!("haiiiii {} :3", hostname))));
            }
            ClientMessage::Unknown(msg) => {
                println!("{}: '{}'", client.address, msg);
                client.writeln(ServerMessage::CommandNotRecognised);
                client.disconnect();
            }
            ClientMessage::StartMail(from) => {
                if let Some(host) = &client.host {
                    println!("{}: mail from {}", host, from);
                    client.writeln(ServerMessage::Okay(None))
                }
            }
        }
    }
    /*
https://datatracker.ietf.org/doc/html/rfc5321#appendix-D.1:
      S: 220 foo.com Simple Mail Transfer Service Ready
      C: EHLO bar.com
      S: 250-foo.com greets bar.com
      S: 250-8BITMIME
      S: 250-SIZE
      S: 250-DSN
      S: 250 HELP
      C: MAIL FROM:<Smith@bar.com>
      S: 250 OK
      C: RCPT TO:<Jones@foo.com>
      S: 250 OK
      C: RCPT TO:<Green@foo.com>
      S: 550 No such user here
      C: RCPT TO:<Brown@foo.com>
      S: 250 OK
      C: DATA
      S: 354 Start mail input; end with <CRLF>.<CRLF>
      C: Blah blah blah...
      C: ...etc. etc. etc.
      C: .
      S: 250 OK
      C: QUIT
      S: 221 foo.com Service closing transmission channel
    */
}