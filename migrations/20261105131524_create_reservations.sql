CREATE TYPE reservation_status AS ENUM ('active', 'expired', 'purchased');

CREATE TABLE reservations (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ticket_offer_id BIGINT NOT NULL REFERENCES ticket_offers (id),
    customer_id BIGINT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    status reservation_status NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
