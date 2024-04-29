# Email Clients for rust

[![Crates.io](https://img.shields.io/crates/v/email-clients.svg)](https://crates.io/crates/email-clients)
[![Documentation](https://docs.rs/email-clients/badge.svg)](https://docs.rs/email-clients)
[![codecov](https://codecov.io/gh/amritghimire/email-clients/branch/master/graph/badge.svg?token=PDN1QX9TVO)](https://codecov.io/gh/amritghimire/email-clients)

## Description

This provides user with easy to use email clients collection in rust. You can choose one or more of the email client and use this library to send emails easily.


## Features

- Terminal client for local development
- Memory client for test cases
- SMTP client with tls and starttls, local support
- Mailersend configuration
- Easy configuration management

## Installation
You can add this library to your project using
```shell
$ cargo add email_clients
```

## Usage

For quick start, you can do the following: 
Based on the email client you want to support, you need to initialize email configuration as below:
```rust
async fn send_email() {
    let email = EmailObject {
        sender: "test@example.com",
        to: vec![EmailAddress { name: "Mail".to_string(), email: "to@example.com".to_string() }],
        subject: "subject".to_string(),
        plain: "plain body".to_string(),
        html: "<a>html body</a>".to_string(),
    };
    
    // Choose any of the config as below:
    // 1. Terminal client (needs terminal feature, enabled by default)
    let terminal_config: TerminalConfig = String::from("me@domain.com").into(); // Terminal config
    // 2. Smtp config (needs smtp feature)
    let smtp_config = SmtpConfig::default().sender("sender@example.com").relay("localhost");
    // 3. Memory config (needs memory feature)
    let (tx, rx) = mpsc::sync_channel(2);
    let memory_config = String::from("me@domain.com").into();
    
    let email_configuration: EmailConfiguration = terminal_config.into(); // OR any of the other config
    let client = get_email_client(email_configuration);
    client.send_emails(email).await;
    
    // For memory config, if you want to retain the receiver, you can do so using:
    let memory_client = EmailClient::Memory(MemoryClient::with_tx(memory_config, tx));
}
```

### Testing
The tests here needs an open mail server listening locally on port 2525. You can do so using:
```shell
$ python -m smtpd -n -c DebuggingServer 127.0.0.1:2525
```
