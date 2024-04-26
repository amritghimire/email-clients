#[cfg(feature = "smtp")]
use crate::clients::smtp;

#[cfg(feature = "terminal")]
use crate::clients::terminal;

#[cfg(feature = "memory")]
use crate::clients::memory;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum EmailConfiguration {
    #[cfg(feature = "terminal")]
    Terminal(terminal::TerminalConfig), // Output to terminal (sender)
    #[cfg(feature = "smtp")]
    SMTP(smtp::SmtpConfig), // Use smtp passwords and options (all config)
    #[cfg(feature = "memory")]
    Memory(memory::MemoryConfig), // Use in memory client
}

#[cfg(feature = "terminal")]
impl Default for EmailConfiguration {
    fn default() -> Self {
        Self::Terminal(terminal::TerminalConfig::default())
    }
}
