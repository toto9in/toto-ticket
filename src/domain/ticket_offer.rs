pub enum TicketType {
    FullPrice,
    HalfPrice,
}

pub enum TicketZone {
    GeneralAdmission,
    Bleachers,
    VipBox,
}

#[derive(Debug, PartialEq)]
pub enum ReservationError {
    InsufficientQuantity,
}

pub struct TicketOffer {
    pub id: u64,
    pub ticket_type: TicketType,
    pub tier: u8,
    pub zone: TicketZone,
    pub quantity: u32,
    pub price_in_cents: u32,
}

impl TicketOffer {
    pub fn reserve(&mut self) -> Result<(), ReservationError> {
        if self.quantity == 0 {
            return Err(ReservationError::InsufficientQuantity);
        }

        self.quantity -= 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_reservation_scenario() {
        let mut ticket_offer = TicketOffer {
            id: 1,
            ticket_type: TicketType::FullPrice,
            tier: 1,
            zone: TicketZone::VipBox,
            quantity: 50,
            price_in_cents: 25000,
        };

        assert!(ticket_offer.reserve().is_ok());
        assert_eq!(ticket_offer.quantity, 49);
    }

    #[test]
    fn failure_reservation_scenario() {
        let mut ticket_offer = TicketOffer {
            id: 2,
            ticket_type: TicketType::FullPrice,
            tier: 1,
            zone: TicketZone::VipBox,
            quantity: 0,
            price_in_cents: 25000,
        };

        assert_eq!(
            ticket_offer.reserve(),
            Err(ReservationError::InsufficientQuantity),
        );
    }
}
