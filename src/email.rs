#[cfg(feature = "smtp")]
use crate::errors::EmailError;
#[cfg(feature = "smtp")]
use lettre::message::Mailbox;
use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct EmailAddress {
    pub name: String,
    pub email: String,
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            write!(f, "{}", self.email)
        } else {
            write!(f, "{} <{}>", self.name, self.email)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct EmailObject {
    pub sender: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub subject: String,
    pub plain: String,
    pub html: String,
}

#[cfg(feature = "smtp")]
impl TryInto<Mailbox> for EmailAddress {
    type Error = EmailError;

    fn try_into(self) -> Result<Mailbox, Self::Error> {
        Ok(Mailbox {
            name: Some(self.name),
            email: self.email.parse()?,
        })
    }
}

impl From<&str> for EmailAddress {
    fn from(value: &str) -> Self {
        Self {
            name: "".to_string(),
            email: value.to_string(),
        }
    }
}
