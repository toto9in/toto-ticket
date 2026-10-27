use std::sync::Arc;

use crate::application::ports::{RepositoryError, TicketOfferRepository};
use crate::domain::ticket_offer::{
    InvalidTicketOffer, NewTicketOffer, TicketOffer, TicketType, TicketZone,
};

pub struct CreateTicketOfferInput {
    pub ticket_type: TicketType,
    pub tier: u8,
    pub zone: TicketZone,
    pub quantity: u32,
    pub price_in_cents: u32,
}

#[derive(Debug, PartialEq)]
pub enum CreateTicketOfferError {
    Invalid(InvalidTicketOffer),
    Repository(RepositoryError),
}

impl From<InvalidTicketOffer> for CreateTicketOfferError {
    fn from(error: InvalidTicketOffer) -> Self {
        Self::Invalid(error)
    }
}

impl From<RepositoryError> for CreateTicketOfferError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}

pub struct CreateTicketOffer {
    repo: Arc<dyn TicketOfferRepository>,
}

impl CreateTicketOffer {
    pub fn new(repo: Arc<dyn TicketOfferRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        input: CreateTicketOfferInput,
    ) -> Result<TicketOffer, CreateTicketOfferError> {
        let offer = NewTicketOffer::new(
            input.ticket_type,
            input.tier,
            input.zone,
            input.quantity,
            input.price_in_cents,
        )?;

        Ok(self.repo.create(offer).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;

    /// In-memory repository that hands out sequential ids.
    #[derive(Default)]
    pub struct FakeTicketOfferRepository {
        pub created: Mutex<Vec<TicketOffer>>,
    }

    #[async_trait]
    impl TicketOfferRepository for FakeTicketOfferRepository {
        async fn create(&self, offer: NewTicketOffer) -> Result<TicketOffer, RepositoryError> {
            let mut created = self
                .created
                .lock()
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            let id = created.len() as u64 + 1;
            let offer = offer.with_id(id);
            created.push(offer.clone());

            Ok(offer)
        }
    }

    fn input(quantity: u32) -> CreateTicketOfferInput {
        CreateTicketOfferInput {
            ticket_type: TicketType::FullPrice,
            tier: 1,
            zone: TicketZone::VipBox,
            quantity,
            price_in_cents: 25000,
        }
    }

    #[tokio::test]
    async fn creates_ticket_offer() {
        let repo = Arc::new(FakeTicketOfferRepository::default());
        let use_case = CreateTicketOffer::new(repo.clone());

        let offer = use_case.execute(input(50)).await;

        assert_eq!(offer.map(|o| (o.id, o.quantity)), Ok((1, 50)));
        assert_eq!(repo.created.lock().map(|c| c.len()).ok(), Some(1));
    }

    #[tokio::test]
    async fn invalid_input_is_not_persisted() {
        let repo = Arc::new(FakeTicketOfferRepository::default());
        let use_case = CreateTicketOffer::new(repo.clone());

        let offer = use_case.execute(input(0)).await;

        assert_eq!(
            offer,
            Err(CreateTicketOfferError::Invalid(
                InvalidTicketOffer::Quantity
            ))
        );
        assert_eq!(repo.created.lock().map(|c| c.len()).ok(), Some(0));
    }
}
