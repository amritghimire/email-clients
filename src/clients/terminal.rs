use crate::configuration::EmailConfiguration;
use crate::email::EmailObject;
use crate::traits::EmailTrait;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialOrd, PartialEq)]
pub struct TerminalConfig {
    pub sender: String,
}

impl From<String> for TerminalConfig {
    /// Converts a `String` into a `MemoryConfig`.
    ///
    /// # Parameters
    /// - `value`: A `String` value which will be used to initialize a `MemoryConfig` instance.
    ///
    /// # Returns
    /// A new instance of `MemoryConfig` with the `sender` property set to the provided `String`.
    ///
    /// # Examples
    /// ```rust
    /// use email_clients::clients::terminal::TerminalConfig;
    /// let value = String::from("sender@example.com");
    /// let config = TerminalConfig::from(value);
    /// assert_eq!(config.sender, "sender@example.com");
    /// ```
    fn from(value: String) -> Self {
        Self { sender: value }
    }
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct TerminalClient {
    sender: String,
}

impl From<TerminalConfig> for EmailConfiguration {
    /// This module manages the Terminal Configuration for an Email Client. It can be used to generate a client from a configuration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use email_clients::email::EmailObject;
    /// # use email_clients::clients::terminal::TerminalConfig;
    /// # use email_clients::configuration::EmailConfiguration;
    /// #
    /// let config: TerminalConfig = String::from("me@domain.com").into();
    /// let email_config: EmailConfiguration = config.into();
    ///```
    ///
    /// Converting a Terminal Configuration into an Email Configuration directly:
    ///
    /// ```
    /// # use email_clients::clients::terminal::TerminalConfig;
    /// # use email_clients::configuration::EmailConfiguration;
    /// #
    /// let config: TerminalConfig = String::from("me@domain.com").into();
    /// let new_config: EmailConfiguration = config.into();
    /// ```
    ///
    fn from(value: TerminalConfig) -> Self {
        EmailConfiguration::Terminal(value)
    }
}

impl TerminalClient {
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            sender: config.sender,
        }
    }
}

#[async_trait]
impl EmailTrait for TerminalClient {
    fn get_sender(&self) -> String {
        self.sender.to_string()
    }

    async fn send_emails(&self, email: EmailObject) -> crate::Result<()> {
        println!("From: {}", self.sender);
        for e in email.to {
            println!("To: {} <{}>", e.name, e.email);
        }
        println!("Subject: {}\n\n", email.subject);
        println!("{}", email.plain);
        println!("----------");
        println!("{}", email.html);
        Ok(())
    }
}
