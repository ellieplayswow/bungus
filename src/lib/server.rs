use std::net::{TcpListener};
use crate::lib::client::{Client};
use crate::lib::message::{ClientMessage, ServerMessage};

pub struct Server {
    host: String,
    port: u16,
    listener: Option<TcpListener>,

    on_connect: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            host: String::from("127.0.0.1"),
            port: 25,
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
                    if let Ok(address) = stream.peer_addr() {
                        println!("New connection: {}", address);

                        let mut client = Client {
                            stream,
                            host: None,

                            address,
                            introduced: false,
                            mail_transport: None,
                            in_data_section: false,
                        };

                        if !client.introduced {
                            client.writeln(ServerMessage::ServiceReady);
                            client.introduced = true;
                        }

                        for line in client.read() {
                            if let Ok(line) = line {
                                Self::handle_message(&mut client, ClientMessage::from(line));
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
        // @todo - introduce client states instead of in_data_section / introduced
        match message {
            ClientMessage::Helo(hostname) => {
                println!("client {} identifying as {}", client.address, hostname);
                client.hostname(hostname.clone());
                client.writeln(ServerMessage::Okay(Some(format!("haiiiii {} :3", hostname))));
            }
            ClientMessage::StartMail(from) => {
                if let Some(host) = &client.host {
                    println!("{}: mail from {}", host, from);
                    client.start_mail(from);
                    client.writeln(ServerMessage::Okay(None))
                }
            }
            ClientMessage::AddRecipient(to) => {
                if let Some(host) = &client.host {
                    println!("{}: mail to {}", host, to);

                    client.add_recipient(to);
                    client.writeln(ServerMessage::Okay(None))
                }
            }
            ClientMessage::BeginData => {
                if let Some(host) = &client.host {
                    println!("{}: beginning data section", host);
                    client.in_data_section = true;

                    client.writeln(ServerMessage::BeginData)
                }
            }

            ClientMessage::Unknown(msg) => {
                println!("{}: '{}'", client.address, msg);
                if client.in_data_section {
                    if msg == "." {
                        client.in_data_section = false;
                        client.writeln(ServerMessage::Okay(None));

                        if let Some(mail_transport) = &client.mail_transport {
                            println!("{:?}", std::str::from_utf8(&mail_transport.data.as_slice()));
                        }
                        // @todo: send mail somewhere
                    }
                    else {
                        client.append_data(msg.as_bytes());
                    }
                }
                else {
                    client.writeln(ServerMessage::CommandNotRecognised);
                }
            }
            ClientMessage::Quit => {
                client.disconnect();
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