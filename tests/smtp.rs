#[cfg(feature = "smtp")]
mod test {
    use secrecy::Secret;

    use email_clients::clients::get_email_client;
    use email_clients::clients::smtp::{SmtpConfig, TlsMode};
    use email_clients::configuration::EmailConfiguration;
    use email_clients::email::{EmailAddress, EmailObject};

    #[tokio::test]
    async fn send_email_saved_in_memory() {
        let recipient_mail = "mail@example.com".to_string();
        let mail_subject = "New subject".to_string();
        let mail_body = "Body of email".to_string();
        let mail_html = "Body of email in <b>HTML</b>".to_string();

        let smtp_config = SmtpConfig {
            sender: "from@example.com".to_string(),
            relay: "127.0.0.1".to_string(),
            username: "".to_string(),
            password: Secret::from("".to_string()),
            port: 2525,
            tls: TlsMode::Local,
        };
        let email_configuration = EmailConfiguration::SMTP(smtp_config);
        let email_client = get_email_client(email_configuration);
        let email = EmailObject {
            sender: "test@example.com".to_string(),
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
}
