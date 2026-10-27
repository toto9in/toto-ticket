use async_trait::async_trait;

use crate::domain::ticket_offer::{NewTicketOffer, TicketOffer};

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    Unexpected(String),
}

#[async_trait]
pub trait TicketOfferRepository: Send + Sync {
    async fn create(&self, offer: NewTicketOffer) -> Result<TicketOffer, RepositoryError>;
}
