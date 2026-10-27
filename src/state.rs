use std::sync::Arc;

use crate::application::create_ticket_offer::CreateTicketOffer;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub create_ticket_offer: Arc<CreateTicketOffer>,
}

impl AppState {
    pub fn new(create_ticket_offer: Arc<CreateTicketOffer>) -> Self {
        Self {
            app_name: "toto-ticket".to_string(),
            create_ticket_offer,
        }
    }
}
