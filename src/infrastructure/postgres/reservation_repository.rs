use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use super::ticket_offer_repository::{lock_ticket_offer, update_quantity};
use super::unexpected;
use crate::application::ports::{RepositoryError, ReservationRepository};
use crate::domain::reservation::{NewReservation, Reservation, ReservationStatus};

pub struct PgReservationRepository {
    pool: PgPool,
}

impl PgReservationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReservationRepository for PgReservationRepository {
    async fn reserve(&self, reservation: NewReservation) -> Result<Reservation, RepositoryError> {
        let mut transaction = self.pool.begin().await.map_err(unexpected)?;

        let mut offer = lock_ticket_offer(&mut transaction, reservation.ticket_offer_id()).await?;
        offer
            .reserve(reservation.quantity())
            .map_err(|error| RepositoryError::Unexpected(format!("{error:?}")))?;
        update_quantity(&mut transaction, &offer).await?;
        let id = insert(&mut transaction, &reservation).await?;

        transaction.commit().await.map_err(unexpected)?;

        Ok(reservation.with_id(id))
    }
}

async fn insert(
    connection: &mut PgConnection,
    reservation: &NewReservation,
) -> Result<u64, RepositoryError> {
    let ticket_offer_id = i64::try_from(reservation.ticket_offer_id()).map_err(unexpected)?;
    let customer_id = i64::try_from(reservation.customer_id()).map_err(unexpected)?;
    let quantity = i32::try_from(reservation.quantity()).map_err(unexpected)?;

    let id: i64 = sqlx::query_scalar(
        "INSERT INTO reservations (ticket_offer_id, customer_id, quantity, status, expires_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id",
    )
    .bind(ticket_offer_id)
    .bind(customer_id)
    .bind(quantity)
    .bind(ReservationStatusRow::from(reservation.status()))
    .bind(reservation.expires_at())
    .fetch_one(connection)
    .await
    .map_err(unexpected)?;

    u64::try_from(id).map_err(unexpected)
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "reservation_status", rename_all = "snake_case")]
enum ReservationStatusRow {
    Active,
    Expired,
    Purchased,
}

impl From<ReservationStatus> for ReservationStatusRow {
    fn from(status: ReservationStatus) -> Self {
        match status {
            ReservationStatus::Active => Self::Active,
            ReservationStatus::Expired => Self::Expired,
            ReservationStatus::Purchased => Self::Purchased,
        }
    }
}
