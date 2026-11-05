use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};

use crate::application::create_reservation::CreateReservationInput;
use crate::domain::reservation::{Reservation, ReservationStatus};
use crate::http::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ReservationStatusDto {
    Active,
    Expired,
    Purchased,
}

#[derive(Deserialize)]
pub struct CreateReservationRequest {
    customer_id: u64,
    quantity: u32,
}

#[derive(Serialize)]
pub struct ReservationResponse {
    id: u64,
    ticket_offer_id: u64,
    customer_id: u64,
    quantity: u32,
    status: ReservationStatusDto,
    expires_at: String,
}

pub async fn create(
    State(state): State<AppState>,
    Path(ticket_offer_id): Path<u64>,
    Json(request): Json<CreateReservationRequest>,
) -> Result<(StatusCode, Json<ReservationResponse>), ApiError> {
    let reservation = state
        .create_reservation
        .execute(CreateReservationInput {
            ticket_offer_id,
            customer_id: request.customer_id,
            quantity: request.quantity,
        })
        .await?;

    Ok((StatusCode::CREATED, Json(reservation.into())))
}

impl From<ReservationStatus> for ReservationStatusDto {
    fn from(status: ReservationStatus) -> Self {
        match status {
            ReservationStatus::Active => Self::Active,
            ReservationStatus::Expired => Self::Expired,
            ReservationStatus::Purchased => Self::Purchased,
        }
    }
}

impl From<Reservation> for ReservationResponse {
    fn from(reservation: Reservation) -> Self {
        Self {
            id: reservation.id,
            ticket_offer_id: reservation.ticket_offer_id,
            customer_id: reservation.customer_id,
            quantity: reservation.quantity,
            status: reservation.status.into(),
            expires_at: reservation
                .expires_at
                .to_rfc3339_opts(SecondsFormat::AutoSi, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, http::Request};
    use chrono::{DateTime, Utc};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use crate::application::create_reservation::CreateReservation;
    use crate::application::create_reservation::tests::FakeReservationRepository;
    use crate::application::create_ticket_offer::CreateTicketOffer;
    use crate::application::create_ticket_offer::tests::FakeTicketOfferRepository;
    use crate::domain::reservation::RESERVATION_DURATION;
    use crate::domain::ticket_offer::{TicketOffer, TicketType, TicketZone};
    use crate::http::router;

    use super::*;

    type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

    fn ticket_offer(id: u64, quantity: u32) -> TicketOffer {
        TicketOffer {
            id,
            ticket_type: TicketType::FullPrice,
            tier: 1,
            zone: TicketZone::VipBox,
            quantity,
            price_in_cents: 25000,
        }
    }

    async fn post(
        repo: Arc<FakeReservationRepository>,
        ticket_offer_id: u64,
        body: Value,
    ) -> TestResult<(StatusCode, Value)> {
        let state = AppState::new(
            Arc::new(CreateTicketOffer::new(Arc::new(
                FakeTicketOfferRepository::default(),
            ))),
            Arc::new(CreateReservation::new(repo)),
        );

        let request = Request::post(format!("/ticket-offers/{ticket_offer_id}/reservations"))
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))?;
        let response = router(state).oneshot(request).await?;

        let status = response.status();
        let bytes = response.into_body().collect().await?.to_bytes();

        Ok((status, serde_json::from_slice(&bytes)?))
    }

    #[tokio::test]
    async fn reserves_tickets_from_a_ticket_offer() -> TestResult {
        let repo = Arc::new(FakeReservationRepository::with_ticket_offer(ticket_offer(
            3, 50,
        )));
        let requested_at = Utc::now();

        let (status, body) = post(repo, 3, json!({ "customer_id": 42, "quantity": 2 })).await?;

        let expires_at: DateTime<Utc> = body["expires_at"].as_str().unwrap_or_default().parse()?;
        assert_eq!(status, StatusCode::CREATED);
        assert!(expires_at >= requested_at + RESERVATION_DURATION);
        assert!(expires_at <= Utc::now() + RESERVATION_DURATION);
        assert_eq!(
            body,
            json!({
                "id": 1,
                "ticket_offer_id": 3,
                "customer_id": 42,
                "quantity": 2,
                "status": "active",
                "expires_at": body["expires_at"]
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn reserving_takes_the_quantity_from_the_ticket_offer() -> TestResult {
        let repo = Arc::new(FakeReservationRepository::with_ticket_offer(ticket_offer(
            3, 50,
        )));

        post(repo.clone(), 3, json!({ "customer_id": 42, "quantity": 2 })).await?;

        let quantities: Vec<u32> = repo
            .ticket_offers
            .lock()
            .map(|offers| offers.iter().map(|offer| offer.quantity).collect())
            .unwrap_or_default();
        assert_eq!(quantities, vec![48]);

        Ok(())
    }

    #[tokio::test]
    async fn rejects_zero_quantity_without_reserving() -> TestResult {
        let repo = Arc::new(FakeReservationRepository::with_ticket_offer(ticket_offer(
            3, 50,
        )));

        let (status, body) =
            post(repo.clone(), 3, json!({ "customer_id": 42, "quantity": 0 })).await?;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body, json!({ "error": "quantity must be greater than 0" }));
        assert_eq!(repo.reservations.lock().map(|r| r.len()).ok(), Some(0));

        Ok(())
    }
}
