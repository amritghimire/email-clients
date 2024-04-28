use crate::email::{EmailAddress, EmailObject};
use async_trait::async_trait;

#[async_trait]
pub trait EmailTrait {
    /// `EmailTrait` is a trait that outlines the basic capabilities for emailing.
    /// It includes capabilities for getting the sender's email address and sending emails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use email_clients::email::{EmailAddress, EmailObject};
    /// use async_trait::async_trait;
    ///
    /// #[async_trait]
    /// pub trait EmailTrait {
    ///     // Retrieves the sender's email address.
    ///     fn get_sender(&self) -> EmailAddress;
    ///
    ///     // Sends an email.
    ///     async fn send_emails(&self, email: EmailObject) -> email_clients::Result<()> {
    ///         // Supposing we have a send_email method in our EmailObject.
    ///         // Ok(self.send_email(email)?)
    ///         Ok(())
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This trait must be implemented by all email utility classes.
    /// - An instance of `EmailObject` passed to `send_emails` method should be a valid EmailObject
    fn get_sender(&self) -> EmailAddress;
    async fn send_emails(&self, email: EmailObject) -> crate::Result<()>;
}
