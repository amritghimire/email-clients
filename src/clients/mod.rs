use crate::configuration::EmailConfiguration;
use crate::traits::EmailTrait;

#[cfg(feature = "smtp")]
pub mod smtp;

#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "terminal")]
pub mod terminal;

///`EmailClient` Enum representing different types of email clients.
///Currently supported email clients: SMTP, Terminal, Memory.
///
/// # Examples
///
/// To integrate SMTP email client:
///
///```Rust
///# use crate::configuration::EmailConfiguration::SMTP;
///# use crate::crate::smtp::SmtpClient;
///let config = SMTP {  /*specify your SMTP configuration here*/ };
///let smtp_email_client = EmailClient::Smtp(SmtpClient::new(config));
///```
///
///To integrate Terminal email client:
///
///```Rust
///# use crate::configuration::EmailConfiguration::Terminal;
///# use crate::crate::terminal::TerminalClient;
///let config = Terminal {  /*specify your Terminal configuration here*/ };
///let terminal_email_client = EmailClient::Terminal(TerminalClient::new(config));
///```
///
///To integrate Memory email client:
///
///```Rust
///# use crate::configuration::EmailConfiguration::Memory;
///# use crate::crate::memory::MemoryClient;
///let config = Memory {  /*specify your Memory configuration here*/ };
///let memory_email_client = EmailClient::Memory(MemoryClient::new(config));
///```
///
#[derive(Clone, Debug)]
pub enum EmailClient {
    #[cfg(feature = "smtp")]
    Smtp(smtp::SmtpClient),
    #[cfg(feature = "terminal")]
    Terminal(terminal::TerminalClient),
    #[cfg(feature = "memory")]
    Memory(memory::MemoryClient),
}

#[cfg(feature = "terminal")]
impl Default for EmailClient {
    /// Default constructor for EmailClient.
    ///
    /// This method will create a new EmailClient configured with default `TerminalClient`.
    /// This is useful when you want to quickly get a working EmailClient without specifying any configuration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use email_clients::clients::EmailClient;
    /// let client = EmailClient::default();
    /// assert_eq!(client.unwrap().get_sender(), "");
    /// ```
    fn default() -> Self {
        EmailClient::Terminal(Default::default())
    }
}

pub fn get_email_client(configuration: EmailConfiguration) -> EmailClient {
    match configuration {
        #[cfg(feature = "terminal")]
        EmailConfiguration::Terminal(c) => EmailClient::Terminal(terminal::TerminalClient::new(c)),
        #[cfg(feature = "smtp")]
        EmailConfiguration::SMTP(smtp_config) => {
            EmailClient::Smtp(smtp::SmtpClient::new(smtp_config))
        }
        #[cfg(feature = "memory")]
        EmailConfiguration::Memory(c) => EmailClient::Memory(memory::MemoryClient::new(c)),
    }
}

impl EmailClient {
    pub fn unwrap(self) -> Box<dyn EmailTrait + Send> {
        match self {
            #[cfg(feature = "smtp")]
            EmailClient::Smtp(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
            #[cfg(feature = "terminal")]
            EmailClient::Terminal(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
            #[cfg(feature = "memory")]
            EmailClient::Memory(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
        }
    }
}
