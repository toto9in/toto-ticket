use std::sync::Arc;

use crate::application::create_reservation::CreateReservation;
use crate::application::create_ticket_offer::CreateTicketOffer;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub create_ticket_offer: Arc<CreateTicketOffer>,
    pub create_reservation: Arc<CreateReservation>,
}

impl AppState {
    pub fn new(
        create_ticket_offer: Arc<CreateTicketOffer>,
        create_reservation: Arc<CreateReservation>,
    ) -> Self {
        Self {
            app_name: "toto-ticket".to_string(),
            create_ticket_offer,
            create_reservation,
        }
    }
}
