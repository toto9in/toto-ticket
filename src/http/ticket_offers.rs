use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::application::create_ticket_offer::CreateTicketOfferInput;
use crate::domain::ticket_offer::{TicketOffer, TicketType, TicketZone};
use crate::http::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum TicketTypeDto {
    FullPrice,
    HalfPrice,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum TicketZoneDto {
    GeneralAdmission,
    Bleachers,
    VipBox,
}

#[derive(Deserialize)]
pub struct CreateTicketOfferRequest {
    ticket_type: TicketTypeDto,
    tier: u8,
    zone: TicketZoneDto,
    quantity: u32,
    price_in_cents: u32,
}

#[derive(Serialize)]
pub struct TicketOfferResponse {
    id: u64,
    ticket_type: TicketTypeDto,
    tier: u8,
    zone: TicketZoneDto,
    quantity: u32,
    price_in_cents: u32,
}

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateTicketOfferRequest>,
) -> Result<(StatusCode, Json<TicketOfferResponse>), ApiError> {
    let offer = state
        .create_ticket_offer
        .execute(CreateTicketOfferInput {
            ticket_type: request.ticket_type.into(),
            tier: request.tier,
            zone: request.zone.into(),
            quantity: request.quantity,
            price_in_cents: request.price_in_cents,
        })
        .await?;

    Ok((StatusCode::CREATED, Json(offer.into())))
}

impl From<TicketTypeDto> for TicketType {
    fn from(dto: TicketTypeDto) -> Self {
        match dto {
            TicketTypeDto::FullPrice => Self::FullPrice,
            TicketTypeDto::HalfPrice => Self::HalfPrice,
        }
    }
}

impl From<TicketType> for TicketTypeDto {
    fn from(ticket_type: TicketType) -> Self {
        match ticket_type {
            TicketType::FullPrice => Self::FullPrice,
            TicketType::HalfPrice => Self::HalfPrice,
        }
    }
}

impl From<TicketZoneDto> for TicketZone {
    fn from(dto: TicketZoneDto) -> Self {
        match dto {
            TicketZoneDto::GeneralAdmission => Self::GeneralAdmission,
            TicketZoneDto::Bleachers => Self::Bleachers,
            TicketZoneDto::VipBox => Self::VipBox,
        }
    }
}

impl From<TicketZone> for TicketZoneDto {
    fn from(zone: TicketZone) -> Self {
        match zone {
            TicketZone::GeneralAdmission => Self::GeneralAdmission,
            TicketZone::Bleachers => Self::Bleachers,
            TicketZone::VipBox => Self::VipBox,
        }
    }
}

impl From<TicketOffer> for TicketOfferResponse {
    fn from(offer: TicketOffer) -> Self {
        Self {
            id: offer.id,
            ticket_type: offer.ticket_type.into(),
            tier: offer.tier,
            zone: offer.zone.into(),
            quantity: offer.quantity,
            price_in_cents: offer.price_in_cents,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use crate::application::create_ticket_offer::CreateTicketOffer;
    use crate::application::create_ticket_offer::tests::FakeTicketOfferRepository;
    use crate::http::router;

    use super::*;

    type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

    async fn post(body: Value) -> TestResult<(StatusCode, Value)> {
        let repo = Arc::new(FakeTicketOfferRepository::default());
        let state = AppState::new(Arc::new(CreateTicketOffer::new(repo)));

        let request = Request::post("/ticket-offers")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))?;
        let response = router(state).oneshot(request).await?;

        let status = response.status();
        let bytes = response.into_body().collect().await?.to_bytes();

        Ok((status, serde_json::from_slice(&bytes)?))
    }

    #[tokio::test]
    async fn creates_ticket_offer() -> TestResult {
        let (status, body) = post(json!({
            "ticket_type": "full_price",
            "tier": 1,
            "zone": "vip_box",
            "quantity": 50,
            "price_in_cents": 25000
        }))
        .await?;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(
            body,
            json!({
                "id": 1,
                "ticket_type": "full_price",
                "tier": 1,
                "zone": "vip_box",
                "quantity": 50,
                "price_in_cents": 25000
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn rejects_invalid_ticket_offer() -> TestResult {
        let (status, body) = post(json!({
            "ticket_type": "half_price",
            "tier": 1,
            "zone": "bleachers",
            "quantity": 0,
            "price_in_cents": 5000
        }))
        .await?;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body, json!({ "error": "quantity must be greater than 0" }));

        Ok(())
    }
}
