use async_trait::async_trait;

use crate::domain::reservation::{NewReservation, Reservation};
use crate::domain::ticket_offer::{NewTicketOffer, TicketOffer};

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    Unexpected(String),
}

#[async_trait]
pub trait TicketOfferRepository: Send + Sync {
    async fn create(&self, offer: NewTicketOffer) -> Result<TicketOffer, RepositoryError>;
}

#[async_trait]
pub trait ReservationRepository: Send + Sync {
    /// Takes the reservation's tickets from its ticket offer and stores it, all or nothing.
    async fn reserve(&self, reservation: NewReservation) -> Result<Reservation, RepositoryError>;
}
