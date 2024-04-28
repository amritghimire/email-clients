use crate::configuration::EmailConfiguration;
use crate::traits::EmailTrait;

#[cfg_attr(docsrs, doc(cfg(feature = "smtp")))]
#[cfg(feature = "smtp")]
pub mod smtp;

#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
#[cfg(feature = "memory")]
pub mod memory;

#[cfg_attr(docsrs, doc(cfg(feature = "terminal")))]
#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg_attr(docsrs, doc(cfg(feature = "mailersend")))]
#[cfg(feature = "mailersend")]
pub mod mailersend;

///`EmailClient` Enum representing different types of email clients.
///Currently supported email clients: SMTP, Terminal, Memory.
///
/// # Examples
///
/// To integrate SMTP email client:
///
///```rust
/// use email_clients::clients::EmailClient;
/// use email_clients::clients::smtp::{SmtpClient, SmtpConfig};
///let config = SmtpConfig::default();
///let smtp_email_client = EmailClient::Smtp(SmtpClient::new(config));
///```
///
///To integrate Terminal email client:
///
///```rust
///# use email_clients::clients::EmailClient;
/// use email_clients::configuration::EmailConfiguration::Terminal;
///# use email_clients::clients::terminal::{TerminalClient, TerminalConfig};
///let config = TerminalConfig::default() ;
///let terminal_email_client = EmailClient::Terminal(TerminalClient::new(config));
///```
///
///To integrate Memory email client:
///
///```rust
/// use email_clients::clients::EmailClient;
/// use email_clients::configuration::EmailConfiguration::Memory;
/// use email_clients::clients::memory::{MemoryClient, MemoryConfig};
///let config = MemoryConfig::default();
///
///let memory_email_client = EmailClient::Memory(MemoryClient::new(config));
///```
///
/// To integrate mailersend client:
///
///```rust
/// use email_clients::clients::EmailClient;
/// use email_clients::clients::mailersend::{MailerSendClient, MailerSendConfig};
/// use email_clients::clients::memory::MemoryClient;
///
/// let config = MailerSendConfig::default().api_token("API_TOKEN");
/// let mailersend_client = EmailClient::MailerSend(MailerSendClient::new(config));
#[derive(Clone, Debug)]
pub enum EmailClient {
    #[cfg(feature = "smtp")]
    Smtp(smtp::SmtpClient),
    #[cfg(feature = "terminal")]
    Terminal(terminal::TerminalClient),
    #[cfg(feature = "memory")]
    Memory(memory::MemoryClient),
    #[cfg(feature = "mailersend")]
    MailerSend(mailersend::MailerSendClient),
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
    /// assert_eq!(client.unwrap().get_sender().to_string(), "");
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
        #[cfg(feature = "mailersend")]
        EmailConfiguration::Mailersend(c) => {
            EmailClient::MailerSend(mailersend::MailerSendClient::new(c))
        }
    }
}

impl EmailClient {
    /// Unwrap the `EmailClient` enum variant and convert it into a `Box<dyn EmailTrait + Send>`.
    ///
    /// This method allows us to obtain a Boxed trait object which implements
    /// `EmailTrait` and `Send` from an instance of `EmailClient` regardless of its variant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use email_clients::clients::EmailClient;
    /// # use email_clients::clients::smtp::{SmtpClient, SmtpConfig};
    ///  use email_clients::email::EmailObject;
    ///  # use email_clients::Result;
    ///
    /// # async fn run() -> Result<()> {
    /// let config = SmtpConfig::default();
    /// let smtp_email_client = EmailClient::Smtp(SmtpClient::new(config));
    ///
    /// // Unwrapping converts the specific variant into a Boxed trait object.
    /// let unwrapped_client = smtp_email_client.unwrap();
    ///
    /// // Now we can use unwrapped_client directly to use methods of EmailTrait.
    /// unwrapped_client.send_emails(EmailObject::default()).await?;
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn unwrap(self) -> Box<dyn EmailTrait + Send> {
        match self {
            #[cfg(feature = "smtp")]
            EmailClient::Smtp(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
            #[cfg(feature = "terminal")]
            EmailClient::Terminal(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
            #[cfg(feature = "memory")]
            EmailClient::Memory(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
            #[cfg(feature = "mailersend")]
            EmailClient::MailerSend(c) => Box::new(c) as Box<dyn EmailTrait + Send>,
        }
    }
}
