use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub enum ReservationStatus {
    Active,
    Expired,
    Purchased,
}

pub struct Reservation {
    pub id: u64,
    pub ticket_offer_id: u64,
    pub customer_id: u64,
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

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    fn reservation_past_its_expiry(status: ReservationStatus) -> Reservation {
        Reservation {
            id: 1,
            ticket_offer_id: 1,
            customer_id: 1,
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
}
