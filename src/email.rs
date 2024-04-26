#[cfg(feature = "smtp")]
use crate::errors::EmailError;
#[cfg(feature = "smtp")]
use lettre::message::Mailbox;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct EmailAddress {
    pub name: String,
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct EmailObject {
    pub sender: String,
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
