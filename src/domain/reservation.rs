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
    use super::*;

    #[test]
    fn active_reservation_expires_after_its_expiry_time() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            customer_id: 1,
            status: ReservationStatus::Active,
            expires_at: ten_minutes_before,
        };

        assert!(reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }

    #[test]
    fn purchased_reservation_stays_purchased_after_its_expiry_time() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            customer_id: 1,
            status: ReservationStatus::Purchased,
            expires_at: ten_minutes_before,
        };

        assert!(!reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Purchased);
    }

    #[test]
    fn expired_reservation_stays_expired() {
        use chrono::TimeDelta;

        let now = Utc::now();
        let one_hour_later = now + TimeDelta::hours(1);
        let ten_minutes_before = now - TimeDelta::minutes(10);

        let mut reservation = Reservation {
            id: 1,
            ticket_offer_id: 1,
            customer_id: 1,
            status: ReservationStatus::Active,
            expires_at: ten_minutes_before,
        };

        assert!(reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);

        assert!(!reservation.try_expire(one_hour_later));
        assert_eq!(reservation.status, ReservationStatus::Expired);
    }
}
