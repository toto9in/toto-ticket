use chrono::{DateTime, Utc};

pub enum OrderStatus {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
}

#[derive(Debug, PartialEq)]
pub enum ReservationStatus {
    Valid,
    Expired,
    Converted,
}

pub struct Reservation {
    pub id: u64,
    pub ticket_offer_id: u64,
    pub client_id: u64,
    pub status: ReservationStatus,
    pub expires_at: DateTime<Utc>,
}

impl Reservation {
    pub fn try_expire(&mut self, time_now: DateTime<Utc>) -> bool {
        if self.status.eq(&ReservationStatus::Valid) && time_now.ge(&self.expires_at) {
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
