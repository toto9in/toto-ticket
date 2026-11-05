use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use super::unexpected;
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

/// Loads a ticket offer and locks its row until the transaction behind `connection` ends.
pub(super) async fn lock_ticket_offer(
    connection: &mut PgConnection,
    id: u64,
) -> Result<TicketOffer, RepositoryError> {
    let id = i64::try_from(id).map_err(unexpected)?;

    let row: TicketOfferRow = sqlx::query_as(
        "SELECT id, ticket_type, tier, zone, quantity, price_in_cents
         FROM ticket_offers
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(id)
    .fetch_one(connection)
    .await
    .map_err(unexpected)?;

    row.try_into()
}

pub(super) async fn update_quantity(
    connection: &mut PgConnection,
    offer: &TicketOffer,
) -> Result<(), RepositoryError> {
    let id = i64::try_from(offer.id).map_err(unexpected)?;
    let quantity = i32::try_from(offer.quantity).map_err(unexpected)?;

    sqlx::query("UPDATE ticket_offers SET quantity = $2 WHERE id = $1")
        .bind(id)
        .bind(quantity)
        .execute(connection)
        .await
        .map_err(unexpected)?;

    Ok(())
}

#[derive(sqlx::FromRow)]
struct TicketOfferRow {
    id: i64,
    ticket_type: TicketTypeRow,
    tier: i16,
    zone: TicketZoneRow,
    quantity: i32,
    price_in_cents: i32,
}

impl TryFrom<TicketOfferRow> for TicketOffer {
    type Error = RepositoryError;

    fn try_from(row: TicketOfferRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u64::try_from(row.id).map_err(unexpected)?,
            ticket_type: row.ticket_type.into(),
            tier: u8::try_from(row.tier).map_err(unexpected)?,
            zone: row.zone.into(),
            quantity: u32::try_from(row.quantity).map_err(unexpected)?,
            price_in_cents: u32::try_from(row.price_in_cents).map_err(unexpected)?,
        })
    }
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

impl From<TicketTypeRow> for TicketType {
    fn from(row: TicketTypeRow) -> Self {
        match row {
            TicketTypeRow::FullPrice => Self::FullPrice,
            TicketTypeRow::HalfPrice => Self::HalfPrice,
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

impl From<TicketZoneRow> for TicketZone {
    fn from(row: TicketZoneRow) -> Self {
        match row {
            TicketZoneRow::GeneralAdmission => Self::GeneralAdmission,
            TicketZoneRow::Bleachers => Self::Bleachers,
            TicketZoneRow::VipBox => Self::VipBox,
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
