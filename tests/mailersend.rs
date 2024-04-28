#[cfg(feature = "mailersend")]
mod test {
    use email_clients::clients::get_email_client;
    use email_clients::clients::mailersend::MailerSendConfig;
    use email_clients::configuration::EmailConfiguration;
    use email_clients::email::{EmailAddress, EmailObject};
    use wiremock::matchers::{bearer_token, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_using_mailersend_success() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/email"))
            .and(bearer_token("API_TOKEN"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1..)
            .mount(&mock_server)
            .await;

        let recipient_mail = "mail@example.com".to_string();
        let mail_subject = "New subject".to_string();
        let mail_body = "Body of email".to_string();
        let mail_html = "Body of email in <b>HTML</b>".to_string();

        let mailersend_config = MailerSendConfig::default()
            .base_url(mock_server.uri())
            .api_token("API_TOKEN")
            .sender("sender@example.com");

        let email_configuration = EmailConfiguration::Mailersend(mailersend_config);
        let email_client = get_email_client(email_configuration);
        let email = EmailObject {
            sender: "test@example.com".into(),
            to: vec![EmailAddress {
                name: "Mail".to_string(),
                email: recipient_mail.clone(),
            }],
            subject: mail_subject.clone(),
            plain: mail_body.clone(),
            html: mail_html,
        };

        email_client
            .unwrap()
            .send_emails(email)
            .await
            .expect("Unable to send email");
    }

    #[tokio::test]
    async fn send_email_using_mailersend_failure() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/email"))
            .and(bearer_token("API_TOKEN"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1..)
            .mount(&mock_server)
            .await;

        let recipient_mail = "mail@example.com".to_string();
        let mail_subject = "New subject".to_string();
        let mail_body = "Body of email".to_string();
        let mail_html = "Body of email in <b>HTML</b>".to_string();

        let mailersend_config = MailerSendConfig::default()
            .base_url(mock_server.uri())
            .api_token("API_TOKEN")
            .sender("sender@example.com");

        let email_configuration = EmailConfiguration::Mailersend(mailersend_config);
        let email_client = get_email_client(email_configuration);
        let email = EmailObject {
            sender: "test@example.com".into(),
            to: vec![EmailAddress {
                name: "Mail".to_string(),
                email: recipient_mail.clone(),
            }],
            subject: mail_subject.clone(),
            plain: mail_body.clone(),
            html: mail_html,
        };

        let response = email_client.unwrap().send_emails(email).await;
        assert!(response.unwrap_err().to_string().starts_with("Failed during making an API request: HTTP status client error (401 Unauthorized) for url"));
    }
}
