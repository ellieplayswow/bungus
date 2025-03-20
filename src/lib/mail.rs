use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Mailbox {
    email: String,
    display_name: Option<String>
}

pub struct MailTransport {
    from: Option<Mailbox>,
    to: Vec<Mailbox>,
    pub data: Vec<u8>
}

impl From<String> for Mailbox {
    fn from(value: String) -> Self {
        let email_start_idx = value.find('<').unwrap_or(0);
        let email_end_idx = value[(email_start_idx + 1)..].find('>').map(|i| i + email_start_idx + 1).unwrap_or(value.len());

        Mailbox {
            email: String::from(&value[(email_start_idx + 1)..email_end_idx]),
            display_name: if email_start_idx == 0 {
                None
            } else {
                Some(String::from(&value[0..(email_start_idx - 1)]))
            }
        }
    }
}

impl Display for Mailbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} <{}>", self.display_name.clone().unwrap_or(String::from("")), self.email))
    }
}

impl MailTransport {
    pub fn new() -> Self {
        MailTransport {
            from: None,
            to: Vec::new(),
            data: Vec::new()
        }
    }
    pub fn from(&mut self, mailbox: Mailbox) {
        self.from = Some(mailbox);
    }

    pub fn to(&mut self, mailbox: Mailbox) {
        self.to.push(mailbox);
    }

    pub fn data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mailbox_parse_email() {
        let expected = Mailbox {
            email: String::from("foobar@example.com"),
            display_name: None,
        };
        let actual = Mailbox::from(String::from("<foobar@example.com>"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn mailbox_parse_display_name() {
        let expected = Mailbox {
            email: String::from("foobar@example.com"),
            display_name: Some(String::from("sir foo of bar")),
        };
        let actual = Mailbox::from(String::from("sir foo of bar <foobar@example.com>"));
        assert_eq!(expected, actual);
    }
}