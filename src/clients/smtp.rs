use crate::configuration::EmailConfiguration;
use crate::email::{EmailAddress, EmailObject};
use crate::traits::EmailTrait;
use async_trait::async_trait;
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SMTP_PORT;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use log::info;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(
    Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize, Default, PartialOrd,
)]
pub enum TlsMode {
    #[default]
    Local,
    Tls,      // Insecure connection only
    StartTls, // Start with insecure connection and use STARTTLS when available
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SmtpConfig {
    pub sender: EmailAddress,
    pub relay: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub tls: TlsMode,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            sender: "".into(),
            relay: "localhost".to_owned(),
            username: "".to_string(),
            port: SMTP_PORT,
            tls: TlsMode::Local,
            password: Secret::from("".to_string()),
        }
    }
}

impl SmtpConfig {
    /// Sets the sender of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::SmtpConfig;
    ///
    /// let mut smtp_config = SmtpConfig::default().sender("Test Sender");
    /// assert_eq!(smtp_config.sender.to_string(), "Test Sender");
    /// ```
    pub fn sender(mut self, value: impl Into<EmailAddress>) -> Self {
        self.sender = value.into();
        self
    }

    /// Sets the relay of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::SmtpConfig;
    ///
    /// let mut smtp_config = SmtpConfig::default().relay("Test Relay");
    /// assert_eq!(smtp_config.relay, "Test Relay");
    /// ```
    pub fn relay(mut self, value: impl AsRef<str>) -> Self {
        self.relay = value.as_ref().to_string();
        self
    }

    /// Sets the username of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::SmtpConfig;
    ///
    /// let mut smtp_config = SmtpConfig::default().username("Test Username");
    /// assert_eq!(smtp_config.username, "Test Username");
    /// ```
    pub fn username(mut self, value: impl AsRef<str>) -> Self {
        self.username = value.as_ref().to_string();
        self
    }

    /// Sets the password of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::SmtpConfig;
    /// use secrecy::{ExposeSecret, Secret};
    ///
    /// let mut smtp_config = SmtpConfig::default().password("Test Password");
    /// assert_eq!(smtp_config.password.expose_secret(), "Test Password");
    /// ```
    pub fn password(mut self, value: impl AsRef<str>) -> Self {
        self.password = Secret::new(value.as_ref().to_string());
        self
    }

    /// Sets the port of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::SmtpConfig;
    ///
    /// let mut smtp_config = SmtpConfig::default().port(1234);
    /// assert_eq!(smtp_config.port, 1234);
    /// ```
    pub fn port(mut self, value: u16) -> Self {
        self.port = value;
        self
    }

    /// Sets the TLS mode of the SMTP config.
    ///
    /// ```
    /// use email_clients::clients::smtp::{SmtpConfig, TlsMode};
    ///
    /// let mut smtp_config = SmtpConfig::default().tls(TlsMode::Tls);
    /// assert_eq!(smtp_config.tls, TlsMode::Tls);
    /// ```
    pub fn tls(mut self, value: TlsMode) -> Self {
        self.tls = value;
        self
    }
}

impl From<SmtpConfig> for EmailConfiguration {
    /// Converts SmtpConfig to EmailConfiguration.
    ///
    /// ```
    /// use email_clients::configuration::EmailConfiguration;
    /// use email_clients::traits::EmailTrait;
    /// use secrecy::Secret;
    /// use email_clients::clients::smtp::{SmtpConfig, TlsMode};
    ///
    /// let smtp_config = SmtpConfig {
    ///     sender: "Test Sender".into(),
    ///     relay: "Test Relay".to_string(),
    ///     username: "Test User".to_string(),
    ///     password: Secret::new("Test Password".to_string()),
    ///     port: 123,
    ///     tls: TlsMode::Local,
    /// };
    ///
    /// let email_config = EmailConfiguration::from(smtp_config);
    /// assert!(matches!(email_config, EmailConfiguration::SMTP(_)));
    /// ```
    fn from(value: SmtpConfig) -> Self {
        EmailConfiguration::SMTP(value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct SmtpClient {
    config: SmtpConfig,
}

impl SmtpClient {
    fn get_transport(&self) -> AsyncSmtpTransport<Tokio1Executor> {
        let settings = &self.config;
        let creds = Credentials::new(
            settings.username.to_owned(),
            settings.password.expose_secret().to_owned(),
        );

        match settings.tls {
            TlsMode::Local => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(settings.relay.as_str())
                    .port(settings.port)
                    .timeout(Some(std::time::Duration::from_secs(10)))
                    .build()
            }
            TlsMode::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(settings.relay.as_str())
                .unwrap()
                .credentials(creds)
                .port(settings.port)
                .build(),
            TlsMode::StartTls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(settings.relay.as_str())
                    .unwrap()
                    .credentials(creds)
                    .port(settings.port)
                    .build()
            }
        }
    }

    pub fn new(config: SmtpConfig) -> Self {
        info!("Starting smtp client");
        Self { config }
    }
}

#[async_trait]
impl EmailTrait for SmtpClient {
    fn get_sender(&self) -> EmailAddress {
        self.config.sender.clone()
    }

    async fn send_emails(&self, email: EmailObject) -> crate::Result<()> {
        let transport = self.get_transport();
        let email_body = MultiPart::alternative_plain_html(email.plain, email.html);

        let mut message_builder = Message::builder()
            .from(self.get_sender().try_into()?)
            .reply_to(self.get_sender().try_into()?);
        for addr in email.to {
            message_builder = message_builder.to(addr.try_into()?)
        }
        let message = message_builder
            .subject(email.subject)
            .multipart(email_body)?;
        transport.send(message).await?;
        Ok(())
    }
}
