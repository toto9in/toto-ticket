use async_trait::async_trait;
use sqlx::PgPool;

use crate::application::ports::{RepositoryError, TicketOfferRepository};
use crate::domain::ticket_offer::{NewTicketOffer, TicketOffer, TicketType, TicketZone};

pub struct PgTicketOfferRepository {
    pool: PgPool,
}

impl PgTicketOfferRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketOfferRepository for PgTicketOfferRepository {
    async fn create(&self, offer: NewTicketOffer) -> Result<TicketOffer, RepositoryError> {
        let quantity = i32::try_from(offer.quantity()).map_err(unexpected)?;
        let price_in_cents = i32::try_from(offer.price_in_cents()).map_err(unexpected)?;

        let id: i64 = sqlx::query_scalar(
            "INSERT INTO ticket_offers (ticket_type, tier, zone, quantity, price_in_cents)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(TicketTypeRow::from(offer.ticket_type()))
        .bind(i16::from(offer.tier()))
        .bind(TicketZoneRow::from(offer.zone()))
        .bind(quantity)
        .bind(price_in_cents)
        .fetch_one(&self.pool)
        .await
        .map_err(unexpected)?;

        let id = u64::try_from(id).map_err(unexpected)?;

        Ok(offer.with_id(id))
    }
}

fn unexpected(error: impl std::fmt::Display) -> RepositoryError {
    RepositoryError::Unexpected(error.to_string())
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "ticket_type", rename_all = "snake_case")]
enum TicketTypeRow {
    FullPrice,
    HalfPrice,
}

impl From<TicketType> for TicketTypeRow {
    fn from(ticket_type: TicketType) -> Self {
        match ticket_type {
            TicketType::FullPrice => Self::FullPrice,
            TicketType::HalfPrice => Self::HalfPrice,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "ticket_zone", rename_all = "snake_case")]
enum TicketZoneRow {
    GeneralAdmission,
    Bleachers,
    VipBox,
}

impl From<TicketZone> for TicketZoneRow {
    fn from(zone: TicketZone) -> Self {
        match zone {
            TicketZone::GeneralAdmission => Self::GeneralAdmission,
            TicketZone::Bleachers => Self::Bleachers,
            TicketZone::VipBox => Self::VipBox,
        }
    }
}
