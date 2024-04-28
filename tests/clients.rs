use email_clients::clients::get_email_client;
#[cfg(feature = "memory")]
use email_clients::clients::memory::MemoryConfig;
#[cfg(feature = "smtp")]
use email_clients::clients::smtp::SmtpConfig;
use email_clients::configuration::EmailConfiguration;

#[cfg(feature = "terminal")]
#[test]
fn test_email_client_terminal() {
    let configuration = EmailConfiguration::default();

    let client = get_email_client(configuration);
    let terminal_client = client.unwrap();

    let sender = terminal_client.get_sender();
    assert_eq!(sender.to_string(), "");
}

#[cfg(feature = "smtp")]
#[test]
fn test_email_client_smtp() {
    let configuration = EmailConfiguration::SMTP(SmtpConfig::default());

    let client = get_email_client(configuration);
    let smtp_client = client.unwrap();

    let sender = smtp_client.get_sender();
    assert_eq!(sender.to_string(), "");
}

#[cfg(feature = "memory")]
#[test]
fn test_email_client_memory() {
    let configuration = EmailConfiguration::Memory(MemoryConfig::default());

    let client = get_email_client(configuration);
    let smtp_client = client.unwrap();

    let sender = smtp_client.get_sender();
    assert_eq!(sender.to_string(), "");
}
