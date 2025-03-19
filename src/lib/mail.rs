use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Mailbox {
    email: String,
    display_name: Option<String>
}

pub struct Mail {
    from: Option<Mailbox>,
    to: Vec<Mailbox>,
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
                Some(String::from(&value[0..email_start_idx]))
            }
        }
    }
}

impl Display for Mailbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} <{}>", self.display_name.clone().unwrap_or(String::from("")), self.email))
    }
}