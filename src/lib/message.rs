use std::fmt::{Display, Formatter};
use crate::lib::mail::Mailbox;

#[derive(Debug)]
pub(crate) enum ClientMessage {
    Helo(String),
    Unknown(String),
    StartMail(Mailbox),
}

pub(crate) enum ServerMessage {
    ServiceReady,
    Okay(Option<String>),

    CommandNotRecognised,
}

impl From<String> for ClientMessage {
    fn from(value: String) -> Self {
        let mut parts = value.split(" ");

        match parts.next() {
            Some(command) => {
                let data = parts.clone().collect::<Vec<&str>>().join(" ");

                match command {
                    "HELO" => ClientMessage::Helo(data),
                    "MAIL"  => {
                        if let Some(recipient) = parts.next() {
                            if recipient.starts_with("FROM:") {
                                let recipient = Mailbox::from(String::from(&data[5..]));
                                return ClientMessage::StartMail(recipient);
                            }
                        }
                        ClientMessage::Unknown(value)
                    }
                    _ => ClientMessage::Unknown(value)
                }
            },
            None => ClientMessage::Unknown(value)
        }
    }
}

impl Display for ServerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerMessage::ServiceReady => f.write_str("220 bungus"),
            ServerMessage::Okay(opt_message) => f.write_fmt(format_args!("250 {}", opt_message.clone().unwrap_or(String::from("bungus")))),
            ServerMessage::CommandNotRecognised => f.write_str("500 command not recognised"),
        }
    }
}