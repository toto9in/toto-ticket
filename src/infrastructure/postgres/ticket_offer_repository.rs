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

#[cfg(test)]
mod tests {
    use super::*;

    /// A `ticket_offers` row with its enums read as text, so tests check what Postgres stored.
    #[derive(Debug, PartialEq, sqlx::FromRow)]
    struct StoredTicketOffer {
        ticket_type: String,
        tier: i16,
        zone: String,
        quantity: i32,
        price_in_cents: i32,
    }

    async fn stored_ticket_offer(pool: &PgPool, id: u64) -> StoredTicketOffer {
        let id = i64::try_from(id).expect("id fits in BIGINT");

        sqlx::query_as(
            "SELECT ticket_type::text AS ticket_type, tier, zone::text AS zone, quantity, price_in_cents
             FROM ticket_offers
             WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .expect("ticket offer row exists")
    }

    #[sqlx::test]
    async fn create_persists_ticket_offer_under_returned_id(pool: PgPool) {
        let repository = PgTicketOfferRepository::new(pool.clone());
        let offer = NewTicketOffer::new(TicketType::HalfPrice, 2, TicketZone::Bleachers, 100, 5000)
            .expect("ticket offer is valid");

        let created = repository
            .create(offer.clone())
            .await
            .expect("ticket offer is created");

        assert_eq!(created, offer.with_id(created.id));
        assert_eq!(
            stored_ticket_offer(&pool, created.id).await,
            StoredTicketOffer {
                ticket_type: "half_price".to_owned(),
                tier: 2,
                zone: "bleachers".to_owned(),
                quantity: 100,
                price_in_cents: 5000,
            }
        );
    }
}
