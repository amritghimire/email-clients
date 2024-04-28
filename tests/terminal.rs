#[cfg(feature = "terminal")]
mod test {
    use email_clients::clients::get_email_client;
    use email_clients::configuration::EmailConfiguration;
    use email_clients::email::{EmailAddress, EmailObject};

    #[tokio::test]
    async fn send_email_in_terminal() {
        let recipient_mail = "mail@example.com".to_string();
        let mail_subject = "New subject".to_string();
        let mail_body = "Body of email".to_string();
        let mail_html = "Body of email in <b>HTML</b>".to_string();

        let terminal_configuration = EmailConfiguration::default();
        let email_client = get_email_client(terminal_configuration);

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
}
