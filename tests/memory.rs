#[cfg(feature = "memory")]
mod test {
    use email_clients::clients::memory::{MemoryClient, MemoryConfig};
    use email_clients::clients::EmailClient;
    use email_clients::email::{EmailAddress, EmailObject};
    use std::sync::mpsc;

    #[tokio::test]
    async fn send_email_saved_in_memory() {
        let recipient_mail = "mail@example.com".to_string();
        let mail_subject = "New subject".to_string();
        let mail_body = "Body of email".to_string();
        let mail_html = "Body of email in <b>HTML</b>".to_string();

        let (tx, rx) = mpsc::sync_channel(2);

        let email_client = EmailClient::Memory(MemoryClient::with_tx(
            MemoryConfig::new("test@example.com".to_string()),
            tx,
        ));
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

        let email = rx.recv().unwrap();

        assert_eq!(email.sender, "test@example.com");
        assert_eq!(email.to[0].email, recipient_mail);
        assert_eq!(email.subject, mail_subject);
        assert_eq!(email.plain, mail_body);
    }
}
