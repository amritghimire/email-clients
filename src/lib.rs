#![cfg_attr(docsrs, feature(doc_cfg))]

//! This provides user with easy to use email clients collection in rust. You can choose one or more of the email client and use this library to send emails easily.
//!
//! Features:
//! * Terminal client for local development
//! * Memory client for test cases
//! * SMTP client with tls and starttls, local support
//! * Easy configuration management
//! * Mailersend client with token and custom base url if needed.
//!
//! # Examples
//!
//! To integrate email client:
//!
//!```rust
//! use std::sync::mpsc;
//! # #[cfg(any(feature = "mailersend", feature = "terminal", feature = "smtp", feature = "memory", feature = "document-features", feature = "default"))]
//! use email_clients::clients::{EmailClient, get_email_client};
//! # #[cfg(feature = "mailersend")]
//! use email_clients::clients::mailersend::MailerSendConfig;
//! # #[cfg(feature = "memory")]
//! use email_clients::clients::memory::{MemoryClient, MemoryConfig};
//! # #[cfg(feature = "smtp")]
//! use email_clients::clients::smtp::SmtpConfig;
//! # #[cfg(feature = "terminal")]
//! use email_clients::clients::terminal::TerminalConfig;
//! use email_clients::configuration::EmailConfiguration;
//! use email_clients::email::{EmailAddress, EmailObject};
//!
//! let email = EmailObject {
//!   sender: "test@example.com".into(),
//!   to: vec![EmailAddress { name: "Mail".to_string(), email: "to@example.com".to_string() }],
//!   subject: "subject".to_string(),
//!   plain: "plain body".to_string(),
//!   html: "<a>html body</a>".to_string(),
//! };
//!
//! // Choose any of the config as below:
//! // 1. Terminal client (needs terminal feature, enabled by default)
//! # #[cfg(feature = "terminal")]
//! let terminal_config: TerminalConfig = String::from("me@domain.com").into(); // Terminal config
//! // 2. Smtp config (needs smtp feature)
//! #[cfg(feature = "smtp")]
//! let smtp_config = SmtpConfig::default().sender("sender@example.com").relay("localhost");
//! // 3. Memory config (needs memory feature)
//! #[cfg(feature = "memory")]
//! let (tx, rx) = mpsc::sync_channel(2);
//! #[cfg(feature = "memory")]
//! let memory_config: MemoryConfig = String::from("me@domain.com").into();
//! // 4. Mailersend config (needs mailersend feature)
//! #[cfg(feature = "mailersend")]
//! let mailersend_config = MailerSendConfig::default().sender("sender@example.com").api_token("API_TOKEN");
//!
//! # #[cfg(feature = "terminal")]
//! # {
//! let email_configuration: EmailConfiguration = terminal_config.into(); // OR any of the other config
//! let client = get_email_client(email_configuration);
//! # tokio_test::block_on(async {
//! client.unwrap().send_emails(email).await.expect("Unable to send email");
//! # });
//! # }
//!
//! // For memory config, if you want to retain the receiver, you can do so using:
//! # #[cfg(feature = "memory")]
//! let memory_client = EmailClient::Memory(MemoryClient::with_tx(memory_config, tx));
//!```
//!
pub mod clients;
pub mod configuration;
pub mod email;
pub mod errors;
pub mod traits;

pub type Result<T> = std::result::Result<T, errors::EmailError>;
