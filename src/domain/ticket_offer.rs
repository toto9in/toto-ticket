#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TicketType {
    FullPrice,
    HalfPrice,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TicketZone {
    GeneralAdmission,
    Bleachers,
    VipBox,
}

#[derive(Debug, PartialEq)]
pub enum ReservationError {
    InsufficientQuantity,
}

#[derive(Debug, PartialEq)]
pub enum InvalidTicketOffer {
    Tier,
    Quantity,
    Price,
}

impl std::fmt::Display for InvalidTicketOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Tier => "tier must be at least 1",
            Self::Quantity => "quantity must be greater than 0",
            Self::Price => "price_in_cents must be greater than 0",
        };

        f.write_str(message)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketOffer {
    pub id: u64,
    pub ticket_type: TicketType,
    pub tier: u8,
    pub zone: TicketZone,
    pub quantity: u32,
    pub price_in_cents: u32,
}

impl TicketOffer {
    pub fn reserve(&mut self, quantity: u32) -> Result<(), ReservationError> {
        if quantity > self.quantity {
            return Err(ReservationError::InsufficientQuantity);
        }

        self.quantity -= quantity;

        Ok(())
    }
}

/// A ticket offer that passed validation but was not persisted yet, so it has no id.
#[derive(Debug, Clone, PartialEq)]
pub struct NewTicketOffer {
    ticket_type: TicketType,
    tier: u8,
    zone: TicketZone,
    quantity: u32,
    price_in_cents: u32,
}

impl NewTicketOffer {
    pub fn new(
        ticket_type: TicketType,
        tier: u8,
        zone: TicketZone,
        quantity: u32,
        price_in_cents: u32,
    ) -> Result<Self, InvalidTicketOffer> {
        if tier == 0 {
            return Err(InvalidTicketOffer::Tier);
        }

        if quantity == 0 {
            return Err(InvalidTicketOffer::Quantity);
        }

        if price_in_cents == 0 {
            return Err(InvalidTicketOffer::Price);
        }

        Ok(Self {
            ticket_type,
            tier,
            zone,
            quantity,
            price_in_cents,
        })
    }

    pub fn ticket_type(&self) -> TicketType {
        self.ticket_type
    }

    pub fn tier(&self) -> u8 {
        self.tier
    }

    pub fn zone(&self) -> TicketZone {
        self.zone
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn price_in_cents(&self) -> u32 {
        self.price_in_cents
    }

    pub fn with_id(self, id: u64) -> TicketOffer {
        TicketOffer {
            id,
            ticket_type: self.ticket_type,
            tier: self.tier,
            zone: self.zone,
            quantity: self.quantity,
            price_in_cents: self.price_in_cents,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ticket_offer_with_quantity(quantity: u32) -> TicketOffer {
        TicketOffer {
            id: 1,
            ticket_type: TicketType::FullPrice,
            tier: 1,
            zone: TicketZone::VipBox,
            quantity,
            price_in_cents: 25000,
        }
    }

    #[test]
    fn reserving_less_than_available_subtracts_the_quantity() {
        let mut ticket_offer = ticket_offer_with_quantity(50);

        let result = ticket_offer.reserve(2);

        assert!(result.is_ok());
        assert_eq!(ticket_offer.quantity, 48);
    }

    #[test]
    fn reserving_exactly_the_available_quantity_leaves_none() {
        let mut ticket_offer = ticket_offer_with_quantity(5);

        let result = ticket_offer.reserve(5);

        assert!(result.is_ok());
        assert_eq!(ticket_offer.quantity, 0);
    }

    #[test]
    fn reserving_more_than_available_fails_and_keeps_the_quantity() {
        let mut ticket_offer = ticket_offer_with_quantity(5);

        let result = ticket_offer.reserve(6);

        assert_eq!(result, Err(ReservationError::InsufficientQuantity));
        assert_eq!(ticket_offer.quantity, 5);
    }

    #[test]
    fn valid_new_ticket_offer() {
        let offer = NewTicketOffer::new(TicketType::HalfPrice, 2, TicketZone::Bleachers, 100, 5000);

        assert!(offer.is_ok());
    }

    #[test]
    fn new_ticket_offer_rejects_tier_zero() {
        let offer = NewTicketOffer::new(TicketType::FullPrice, 0, TicketZone::VipBox, 50, 25000);

        assert_eq!(offer, Err(InvalidTicketOffer::Tier));
    }

    #[test]
    fn new_ticket_offer_rejects_zero_quantity() {
        let offer = NewTicketOffer::new(TicketType::FullPrice, 1, TicketZone::VipBox, 0, 25000);

        assert_eq!(offer, Err(InvalidTicketOffer::Quantity));
    }

    #[test]
    fn new_ticket_offer_rejects_zero_price() {
        let offer = NewTicketOffer::new(TicketType::FullPrice, 1, TicketZone::VipBox, 50, 0);

        assert_eq!(offer, Err(InvalidTicketOffer::Price));
    }
}
