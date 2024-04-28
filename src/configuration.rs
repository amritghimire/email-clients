#[cfg(feature = "smtp")]
use crate::clients::smtp;

#[cfg(feature = "terminal")]
use crate::clients::terminal;

#[cfg(feature = "memory")]
use crate::clients::memory;

#[cfg(feature = "mailersend")]
use crate::clients::mailersend;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum EmailConfiguration {
    #[cfg(feature = "terminal")]
    Terminal(terminal::TerminalConfig), // Output to terminal (sender)
    #[cfg(feature = "smtp")]
    SMTP(smtp::SmtpConfig), // Use smtp passwords and options (all config)
    #[cfg(feature = "memory")]
    Memory(memory::MemoryConfig), // Use in memory client
    #[cfg(feature = "mailersend")]
    Mailersend(mailersend::MailerSendConfig), // Use mailersend client
}

#[cfg(feature = "terminal")]
impl Default for EmailConfiguration {
    fn default() -> Self {
        Self::Terminal(terminal::TerminalConfig::default())
    }
}
