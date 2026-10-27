use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::application::create_ticket_offer::CreateTicketOfferError;

pub struct ApiError {
    status: StatusCode,
    message: String,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: self.message,
            }),
        )
            .into_response()
    }
}

impl From<CreateTicketOfferError> for ApiError {
    fn from(error: CreateTicketOfferError) -> Self {
        match error {
            CreateTicketOfferError::Invalid(error) => Self {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                message: error.to_string(),
            },
            CreateTicketOfferError::Repository(error) => {
                eprintln!("failed to create ticket offer: {error:?}");

                Self {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "internal server error".to_string(),
                }
            }
        }
    }
}
