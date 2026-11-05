mod reservation_repository;
mod ticket_offer_repository;

pub use reservation_repository::PgReservationRepository;
pub use ticket_offer_repository::PgTicketOfferRepository;

use crate::application::ports::RepositoryError;

fn unexpected(error: impl std::fmt::Display) -> RepositoryError {
    RepositoryError::Unexpected(error.to_string())
}
