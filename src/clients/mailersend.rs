use crate::configuration::EmailConfiguration;
use crate::email::{EmailAddress, EmailObject};
use crate::traits::EmailTrait;
use crate::Result;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Method};
use secrecy::{ExposeSecret, Secret};

static BASE_URL: &str = "https://api.mailersend.com/v1";

fn default_base_url() -> String {
    BASE_URL.to_string()
}


/// `MailerSendConfig` structure that includes sender, base_url, and api_token.
///
/// ```rust
/// use email_clients::clients::mailersend::MailerSendConfig;
///
/// let mut mailer_send_config = MailerSendConfig::default()
///                                .sender("sender@example.com")
///                                .base_url("https://api.mailersend.com/v1")
///                                .api_token("test_api_token");
/// assert_eq!(mailer_send_config.get_sender().to_string(), "sender@example.com");
/// assert_eq!(mailer_send_config.get_base_url(), "https://api.mailersend.com/v1");
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MailerSendConfig {
    sender: EmailAddress,
    #[serde(default = "default_base_url")]
    base_url: String,
    api_token: Secret<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct EmailPayload {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    subject: String,
    text: String,
    html: String,
}

impl From<EmailObject> for EmailPayload {
    fn from(value: EmailObject) -> Self {
        Self {
            from: value.sender,
            to: value.to,
            subject: value.subject,
            text: value.plain,
            html: value.html,
        }
    }
}

impl Default for MailerSendConfig {
    /// Constructs a `MailerSendConfig` with default values:
    /// - sender: An empty string `""`
    /// - base_url: `https://api.mailersend.com/v1`
    /// - api_token: An empty string `""`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let config = MailerSendConfig::default();
    ///
    /// assert_eq!(config.get_sender().to_string(), "");
    /// assert_eq!(config.get_base_url(), "https://api.mailersend.com/v1");
    /// ```
    ///
    fn default() -> Self {
        Self {
            sender: "".into(),
            base_url: BASE_URL.to_string(),
            api_token: Secret::from("".to_string()),
        }
    }
}

impl MailerSendConfig {
    /// Sets the sender of the Mailersend config.
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let mut smtp_config = MailerSendConfig::default().sender("Test Sender");
    /// assert_eq!(smtp_config.get_sender().to_string(), "Test Sender");
    /// ```
    pub fn sender(mut self, value: impl Into<EmailAddress>) -> Self {
        self.sender = value.into();
        self
    }

    /// Sets the base_url of the Mailersend config.
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let mut smtp_config = MailerSendConfig::default().base_url("Test URL");
    /// assert_eq!(smtp_config.get_base_url(), "Test URL");
    /// ```
    pub fn base_url(mut self, value: impl AsRef<str>) -> Self {
        self.base_url = value.as_ref().trim_end_matches('/').to_string();
        self
    }

    /// Sets the api_token of the Mailersend config.
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let mut smtp_config = MailerSendConfig::default().api_token("Test Token");
    /// ```
    pub fn api_token(mut self, value: impl AsRef<str>) -> Self {
        self.api_token = Secret::new(value.as_ref().to_string());
        self
    }

    /// Returns the base url of the Mailersend config.
    ///
    /// # Example
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let smtp_config = MailerSendConfig::default().base_url("https://api.mailersend.com/v1");
    /// assert_eq!(smtp_config.get_base_url(), "https://api.mailersend.com/v1");
    /// ```
    ///
    pub fn get_base_url(&self) -> String {
        self.base_url.to_string()
    }

    /// Returns the sender of the Mailersend config.
    ///
    /// # Example
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let mailer_send_config = MailerSendConfig::default().sender("test_sender@example.com");
    /// assert_eq!(mailer_send_config.get_sender().to_string(), "test_sender@example.com");
    /// ```
    ///
    pub fn get_sender(&self) -> EmailAddress {
        self.sender.clone()
    }
}

impl From<MailerSendConfig> for EmailConfiguration {
    /// Converts a `MailerSendConfig` into an `EmailConfiguration`
    ///
    /// This conversion is mainly used when we are setting the configuration for our email client.
    ///
    /// # Example
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    /// use email_clients::configuration::EmailConfiguration;
    ///
    /// let mailer_config = MailerSendConfig::default()
    ///                 .sender("sender@example.com")
    ///                 .base_url("https://api.mailersend.com/v1")
    ///                 .api_token("test_api_token");
    ///
    /// let email_config: EmailConfiguration = mailer_config.into();
    /// ```
    fn from(value: MailerSendConfig) -> Self {
        EmailConfiguration::Mailersend(value)
    }
}

/// `MailerSendClient` structure that includes 'config' and 'reqwest_client'.
///
/// ```rust
/// use email_clients::clients::mailersend::MailerSendConfig;
/// use email_clients::clients::mailersend::MailerSendClient;
///
/// let mailer_send_config = MailerSendConfig::default()
///                            .sender("sender@example.com")
///                            .base_url("https://api.mailersend.com/v1")
///                            .api_token("test_api_token");
/// let mailer_send_client = MailerSendClient::new(mailer_send_config);
/// ```
#[derive(Clone, Debug, Default)]
pub struct MailerSendClient {
    config: MailerSendConfig,
    reqwest_client: Client,
}

impl MailerSendClient {
    pub fn new(config: MailerSendConfig) -> Self {
        let reqwest_client = Client::new();

        MailerSendClient {
            config,
            reqwest_client,
        }
    }

    fn url(&self) -> String {
        format!("{}/email", self.config.base_url.trim_end_matches('/'))
    }

    fn headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.config.api_token.expose_secret()).parse()?,
        );
        Ok(headers)
    }
}

#[async_trait]
impl EmailTrait for MailerSendClient {
    /// Returns the sender included in the `MailerSendClient`'s configuration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use email_clients::clients::mailersend::MailerSendConfig;
    /// use email_clients::clients::mailersend::MailerSendClient;
    /// use email_clients::traits::EmailTrait;
    ///
    /// let mailer_send_config = MailerSendConfig::default()
    ///     .sender("sender@example.com")
    ///     .base_url("https://api.mailersend.com/v1")
    ///     .api_token("test_api_token");
    ///
    /// let mailer_send_client = MailerSendClient::new(mailer_send_config);
    ///
    /// assert_eq!(mailer_send_client.get_sender().to_string(), "sender@example.com");
    /// ```
    fn get_sender(&self) -> EmailAddress {
        self.config.get_sender().clone()
    }

    async fn send_emails(&self, email: EmailObject) -> Result<()> {
        let payload: EmailPayload = email.into();
        self.reqwest_client
            .request(Method::POST, self.url())
            .headers(self.headers()?)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
