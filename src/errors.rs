#[cfg(feature = "smtp")]
use lettre::address::AddressError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[cfg(feature = "smtp")]
    #[error("Invalid address {0}")]
    AddressError(#[from] AddressError),
    #[cfg(feature = "smtp")]
    #[error("Failed to send email {0}")]
    Lettre(#[from] lettre::error::Error),
    #[cfg(feature = "smtp")]
    #[error("Failed to send email {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
    #[cfg(feature = "mailersend")]
    #[error("Invalid api token for mailsend")]
    MailsendHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[cfg(feature = "mailersend")]
    #[error("Failed during making an API request: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
