use crate::configuration::EmailConfiguration;
use crate::email::{EmailAddress, EmailObject};
use crate::traits::EmailTrait;
use crate::Result;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Method};
use secrecy::{ExposeSecret, Secret};

static BASE_URL: &str = "https://api.mailersend.com/v1/";


/// `MailerSendConfig` structure that includes sender, base_url, and api_token.
///
/// ```rust
/// use email_clients::clients::mailersend::MailerSendConfig;
///
/// let mut mailer_send_config = MailerSendConfig::default()
///                                .sender("sender@example.com".to_string())
///                                .base_url("https://api.mailersend.com/v1/")
///                                .api_token("test_api_token");
/// assert_eq!(mailer_send_config.get_sender().to_string(), "sender@example.com");
/// assert_eq!(mailer_send_config.get_base_url(), "https://api.mailersend.com/v1/");
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MailerSendConfig {
    sender: EmailAddress,
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
    fn default() -> Self {
        Self {
            sender: "".to_string().into(),
            base_url: BASE_URL.to_string(),
            api_token: Secret::from("".to_string()),
        }
    }
}

impl MailerSendConfig {
    /// Sets the sender of the Mailersend config.
    ///
    /// ```
    /// use email_clients::clients::mailersend::MailerSendConfig;
    ///
    /// let mut smtp_config = MailerSendConfig::default().sender("Test Sender".to_string());
    /// assert_eq!(smtp_config.get_sender().to_string(), "Test Sender");
    /// ```
    pub fn sender(mut self, value: impl Into<EmailAddress>) -> Self {
        self.sender = value.into();
        self
    }
    pub fn base_url(mut self, value: impl AsRef<str>) -> Self {
        self.base_url = value.as_ref().to_string();
        self
    }

    pub fn api_token(mut self, value: impl AsRef<str>) -> Self {
        self.api_token = Secret::new(value.as_ref().to_string());
        self
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.to_string()
    }

    pub fn get_sender(&self) -> EmailAddress {
        self.sender.clone()
    }
}

impl From<MailerSendConfig> for EmailConfiguration {
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
///                            .sender("sender@example.com".to_string())
///                            .base_url("https://api.mailersend.com/v1/")
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
        format!("{}email", self.config.base_url)
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
    fn get_sender(&self) -> String {
        self.config.get_sender().to_string()
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
