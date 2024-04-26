use crate::email::EmailObject;
use async_trait::async_trait;

#[async_trait]
pub trait EmailTrait {
    fn get_sender(&self) -> String;

    async fn send_emails(&self, email: EmailObject) -> crate::Result<()>;
}
