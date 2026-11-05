use std::sync::Arc;

use chrono::Utc;

use crate::application::ports::{RepositoryError, ReservationRepository};
use crate::domain::reservation::{InvalidReservation, NewReservation, Reservation};

pub struct CreateReservationInput {
    pub ticket_offer_id: u64,
    pub customer_id: u64,
    pub quantity: u32,
}

#[derive(Debug, PartialEq)]
pub enum CreateReservationError {
    Invalid(InvalidReservation),
    Repository(RepositoryError),
}

impl From<InvalidReservation> for CreateReservationError {
    fn from(error: InvalidReservation) -> Self {
        Self::Invalid(error)
    }
}

impl From<RepositoryError> for CreateReservationError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}

pub struct CreateReservation {
    repository: Arc<dyn ReservationRepository>,
}

impl CreateReservation {
    pub fn new(repository: Arc<dyn ReservationRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateReservationInput,
    ) -> Result<Reservation, CreateReservationError> {
        let reservation = NewReservation::new(
            input.ticket_offer_id,
            input.customer_id,
            input.quantity,
            Utc::now(),
        )?;

        Ok(self.repository.reserve(reservation).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::ticket_offer::TicketOffer;

    /// In-memory repository holding ticket offers to reserve from; hands out sequential ids.
    #[derive(Default)]
    pub struct FakeReservationRepository {
        pub ticket_offers: Mutex<Vec<TicketOffer>>,
        pub reservations: Mutex<Vec<Reservation>>,
    }

    impl FakeReservationRepository {
        pub fn with_ticket_offer(offer: TicketOffer) -> Self {
            Self {
                ticket_offers: Mutex::new(vec![offer]),
                ..Self::default()
            }
        }
    }

    fn unexpected(error: impl std::fmt::Debug) -> RepositoryError {
        RepositoryError::Unexpected(format!("{error:?}"))
    }

    #[async_trait]
    impl ReservationRepository for FakeReservationRepository {
        async fn reserve(
            &self,
            reservation: NewReservation,
        ) -> Result<Reservation, RepositoryError> {
            let mut ticket_offers = self.ticket_offers.lock().map_err(unexpected)?;
            let mut reservations = self.reservations.lock().map_err(unexpected)?;

            let offer = ticket_offers
                .iter_mut()
                .find(|offer| offer.id == reservation.ticket_offer_id())
                .ok_or_else(|| unexpected("ticket offer not found"))?;
            offer.reserve(reservation.quantity()).map_err(unexpected)?;

            let reservation = reservation.with_id(reservations.len() as u64 + 1);
            reservations.push(reservation.clone());

            Ok(reservation)
        }
    }
}
