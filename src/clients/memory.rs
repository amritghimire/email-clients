use crate::configuration::EmailConfiguration;
use async_trait::async_trait;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;

use crate::email::EmailObject;
use crate::errors::EmailError;
use crate::traits::EmailTrait;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialOrd, PartialEq)]
pub struct MemoryConfig {
    pub sender: String,
}

impl MemoryConfig {
    /// Generates a new `MemoryConfig`.
    ///
    /// # Parameters
    /// - `sender`: A `String` that will be used as the sender of the `MemoryConfig`.
    ///
    /// # Returns
    /// A new instance of `MemoryConfig` with the `sender` property set to the provided `String`.
    ///
    /// # Examples
    /// ```rust
    /// use email_clients::clients::memory::MemoryConfig;
    ///
    /// let config = MemoryConfig::new("sender@example.com");
    /// assert_eq!(config.sender, "sender@example.com");
    /// ```
    pub fn new(sender: impl AsRef<str>) -> Self {
        Self {
            sender: sender.as_ref().to_string(),
        }
    }
}

impl From<String> for MemoryConfig {
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
    /// use email_clients::clients::memory::MemoryConfig;
    /// let value = String::from("sender@example.com");
    /// let config = MemoryConfig::from(value);
    /// assert_eq!(config.sender, "sender@example.com");
    /// ```
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<MemoryConfig> for EmailConfiguration {
    /// Implementation of From trait to convert MemoryConfig into EmailConfiguration
    ///
    /// # Example
    ///
    /// ```rust
    /// # use email_clients::configuration::EmailConfiguration;
    /// # use email_clients::clients::memory::MemoryConfig;
    /// #
    /// let memory_config = MemoryConfig::new("sender@example.com");
    /// let email_config: EmailConfiguration = memory_config.into();
    /// #
    /// # match email_config {
    /// #    EmailConfiguration::Memory(mc) => {
    /// #       assert_eq!(mc.sender, "sender@example.com");
    /// #   },
    /// #    _ => panic!("Invalid conversion"),
    /// # }
    /// ```
    fn from(value: MemoryConfig) -> Self {
        EmailConfiguration::Memory(value)
    }
}

#[derive(Clone, Debug)]
pub struct MemoryClient {
    sender: String,
    tx: SyncSender<EmailObject>,
}

impl Default for MemoryClient {
    /// `Default` implementation for `MemoryClient`.
    ///
    /// This method will return a `MemoryClient` instance with an empty sender and a `SyncSender<EmailObject>`
    /// with a channel buffer size of 5.
    ///
    /// # Returns
    /// A `MemoryClient` instance with the default configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use email_clients::clients::memory::MemoryClient;
    /// use email_clients::traits::EmailTrait;
    ///
    /// let default_client = MemoryClient::default();
    /// // Gets the default sender which is an empty string
    /// assert_eq!(default_client.get_sender(), "");
    /// ```
    fn default() -> Self {
        let (tx, _) = mpsc::sync_channel(5 /* usize */);
        Self {
            sender: "".to_string(),
            tx,
        }
    }
}

impl MemoryClient {
    /// Initializes a new `MemoryClient` with the provided `MemoryConfig`.
    ///
    /// # Parameters
    /// - `config`: A `MemoryConfig` instance that will be used to initialize the `MemoryClient`.
    ///
    /// # Returns
    /// A new instance of `MemoryClient` with the `sender` and `SyncSender<EmailObject>` set per the provided `MemoryConfig`.
    ///
    /// # Examples
    /// ```rust
    /// # use email_clients::clients::memory::{MemoryConfig, MemoryClient};
    /// # use email_clients::traits::EmailTrait;
    ///
    /// let config = MemoryConfig::new("sender@example.com");
    /// let client = MemoryClient::new(config);
    /// assert_eq!(client.get_sender(), "sender@example.com");
    /// ```
    pub fn new(config: MemoryConfig) -> Self {
        let (tx, _) = mpsc::sync_channel(5 /* usize */);

        Self {
            sender: config.sender,
            tx,
        }
    }

    /// Initializes a new `MemoryClient` with the provided `MemoryConfig` and `SyncSender<EmailObject>`.
    ///
    /// # Parameters
    /// - `config`: A `MemoryConfig` instance that will be used to initialize the `MemoryClient`.
    /// - `tx`: A `SyncSender<EmailObject>` instance that will be used for sending emails.
    ///
    /// # Returns
    /// A new instance of `MemoryClient` with the `sender` and `SyncSender<EmailObject>` set per the provided parameters.
    ///
    /// # Examples
    /// ```rust
    /// # use std::sync::mpsc::sync_channel;
    /// # use email_clients::clients::memory::{MemoryConfig, MemoryClient};
    /// # use email_clients::email::EmailObject;
    /// # use email_clients::traits::EmailTrait;
    ///
    /// let config = MemoryConfig::new("sender@example.com");
    /// let (tx, rx) = sync_channel(2);
    /// let client = MemoryClient::with_tx(config, tx.clone());
    /// assert_eq!(client.get_sender(), "sender@example.com");
    /// ```
    pub fn with_tx(config: MemoryConfig, tx: SyncSender<EmailObject>) -> Self {
        Self {
            sender: config.sender,
            tx,
        }
    }
}

#[async_trait]
impl EmailTrait for MemoryClient {
    fn get_sender(&self) -> String {
        self.sender.to_string()
    }

    async fn send_emails(&self, email: EmailObject) -> crate::Result<()> {
        self.tx
            .send(email)
            .map_err(|_| EmailError::UnexpectedError("Cannot send email in memory".to_string()))?;
        Ok(())
    }
}
