use chrono::{DateTime, Utc};

enum OrderStatus {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
}

#[derive(Debug, PartialEq)]
enum ReservationStatus {
    Valid,
    Expired,
    Converted,
}

struct Reservation {
    id: u64,
    ticket_offer_id: u64,
    client_id: u64,
    status: ReservationStatus,
    expires_at: DateTime<Utc>,
}

impl Reservation {
    fn try_expire(&mut self, time_now: DateTime<Utc>) -> bool {
        if self.status.eq(&ReservationStatus::Valid) && time_now.ge(&self.expires_at) {
            self.status = ReservationStatus::Expired;

            return true;
        }

        false
    }
}

enum TicketType {
    FullPrice,
    HalfPrice,
}

enum TicketZone {
    GeneralAdmission,
    Bleachers,
    VipBox,
}

#[derive(Debug, PartialEq)]
enum ReservationError {
    InsufficientQuantity,
}

struct TicketOffer {
    id: u64,
    ticket_type: TicketType,
    tier: u8,
    zone: TicketZone,
    quantity: u32,
    price_in_cents: u32,
}

impl TicketOffer {
    fn reserve(&mut self) -> Result<(), ReservationError> {
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

    #[test]
    fn reservation_expired_scenario() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            client_id: 1,
            status: ReservationStatus::Valid,
            expires_at: ten_minutes_before,
        };

        assert!(reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }

    #[test]
    fn converted_reservation_stills_converted_after_time_is_expired() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            client_id: 1,
            status: ReservationStatus::Converted,
            expires_at: ten_minutes_before,
        };

        assert!(!reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Converted);
    }

    #[test]
    fn expired_reservation_stills_expired() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            client_id: 1,
            status: ReservationStatus::Valid,
            expires_at: ten_minutes_before,
        };

        assert!(reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);

        assert!(!reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }
}
