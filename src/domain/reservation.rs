use chrono::{DateTime, TimeDelta, Utc};

/// How long a reservation holds its tickets before it expires.
pub const RESERVATION_DURATION: TimeDelta = TimeDelta::minutes(10);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReservationStatus {
    Active,
    Expired,
    Purchased,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reservation {
    pub id: u64,
    pub ticket_offer_id: u64,
    pub customer_id: u64,
    pub quantity: u32,
    pub status: ReservationStatus,
    pub expires_at: DateTime<Utc>,
}

impl Reservation {
    pub fn try_expire(&mut self, time_now: DateTime<Utc>) -> bool {
        if self.status.eq(&ReservationStatus::Active) && time_now.ge(&self.expires_at) {
            self.status = ReservationStatus::Expired;

            return true;
        }

        false
    }
}

#[derive(Debug, PartialEq)]
pub enum InvalidReservation {
    Quantity,
}

impl std::fmt::Display for InvalidReservation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Quantity => "quantity must be greater than 0",
        };

        f.write_str(message)
    }
}

/// A reservation that passed validation but was not persisted yet, so it has no id.
#[derive(Debug, Clone, PartialEq)]
pub struct NewReservation {
    ticket_offer_id: u64,
    customer_id: u64,
    quantity: u32,
    status: ReservationStatus,
    expires_at: DateTime<Utc>,
}

impl NewReservation {
    /// A reservation starts active: its tickets are held until it expires or is purchased.
    pub fn new(
        ticket_offer_id: u64,
        customer_id: u64,
        quantity: u32,
        now: DateTime<Utc>,
    ) -> Result<Self, InvalidReservation> {
        if quantity == 0 {
            return Err(InvalidReservation::Quantity);
        }

        Ok(Self {
            ticket_offer_id,
            customer_id,
            quantity,
            status: ReservationStatus::Active,
            expires_at: now + RESERVATION_DURATION,
        })
    }

    pub fn ticket_offer_id(&self) -> u64 {
        self.ticket_offer_id
    }

    pub fn customer_id(&self) -> u64 {
        self.customer_id
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn status(&self) -> ReservationStatus {
        self.status
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn with_id(self, id: u64) -> Reservation {
        Reservation {
            id,
            ticket_offer_id: self.ticket_offer_id,
            customer_id: self.customer_id,
            quantity: self.quantity,
            status: self.status,
            expires_at: self.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reservation_past_its_expiry(status: ReservationStatus) -> Reservation {
        Reservation {
            id: 1,
            ticket_offer_id: 1,
            customer_id: 1,
            quantity: 1,
            status,
            expires_at: Utc::now() - TimeDelta::minutes(10),
        }
    }

    #[test]
    fn active_reservation_expires_after_its_expiry_time() {
        let mut reservation = reservation_past_its_expiry(ReservationStatus::Active);

        let expired = reservation.try_expire(Utc::now());

        assert!(expired);
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }

    #[test]
    fn purchased_reservation_stays_purchased_after_its_expiry_time() {
        let mut reservation = reservation_past_its_expiry(ReservationStatus::Purchased);

        let expired = reservation.try_expire(Utc::now());

        assert!(!expired);
        assert_eq!(reservation.status, ReservationStatus::Purchased);
    }

    #[test]
    fn expired_reservation_stays_expired() {
        let mut reservation = reservation_past_its_expiry(ReservationStatus::Expired);

        let expired = reservation.try_expire(Utc::now());

        assert!(!expired);
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }

    #[test]
    fn new_reservation_rejects_zero_quantity() {
        let reservation = NewReservation::new(1, 42, 0, Utc::now());

        assert_eq!(reservation, Err(InvalidReservation::Quantity));
    }

    #[test]
    fn new_reservation_expires_ten_minutes_after_it_is_made() -> Result<(), InvalidReservation> {
        let made_at = DateTime::from_timestamp(1_700_000_000, 0).unwrap_or_default();

        let reservation = NewReservation::new(1, 42, 2, made_at)?;

        assert_eq!(
            reservation.expires_at().timestamp(),
            1_700_000_000 + 10 * 60
        );

        Ok(())
    }

    #[test]
    fn persisted_reservation_starts_active() -> Result<(), InvalidReservation> {
        let new_reservation = NewReservation::new(3, 42, 2, Utc::now())?;

        let reservation = new_reservation.with_id(7);

        assert_eq!(
            (
                reservation.id,
                reservation.ticket_offer_id,
                reservation.customer_id
            ),
            (7, 3, 42)
        );
        assert_eq!(reservation.quantity, 2);
        assert_eq!(reservation.status, ReservationStatus::Active);

        Ok(())
    }
}
