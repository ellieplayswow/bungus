use std::fmt::Display;
use std::io::{BufRead, BufReader, Lines, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use crate::lib::mail::{MailTransport, Mailbox};
use crate::lib::message::ServerMessage;

pub struct Client {
    pub(crate) stream: TcpStream,
    pub(crate) host: Option<String>,

    pub address: SocketAddr,
    pub introduced: bool,
    pub mail_transport: Option<MailTransport>,
    pub in_data_section: bool
}

impl Client {
    pub fn read(&self) -> Lines<BufReader<TcpStream>> {
        let reader = BufReader::new(self.stream.try_clone().unwrap());

        reader.lines()
    }

    pub fn start_mail(&mut self, from: Mailbox) {
        let mut mail_transport = MailTransport::new();
        mail_transport.from(from);
        self.mail_transport = Some(mail_transport);
    }

    pub fn add_recipient(&mut self, to: Mailbox) {
        if let Some(ref mut mail_transport) = self.mail_transport {
            mail_transport.to(to);
        }
    }

    pub fn append_data(&mut self, data: &[u8]) {
        if let Some(ref mut mail_transport) = self.mail_transport {
            let mut data = data.to_vec();
            data.push(b'\r');
            data.push(b'\n');
            mail_transport.data(data.as_slice());
        }
    }

    pub fn disconnect(&mut self) {
        self.writeln(ServerMessage::ClosingConnection);
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