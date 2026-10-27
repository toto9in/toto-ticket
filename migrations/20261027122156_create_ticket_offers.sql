CREATE TYPE ticket_type AS ENUM ('full_price', 'half_price');

CREATE TYPE ticket_zone AS ENUM ('general_admission', 'bleachers', 'vip_box');

CREATE TABLE ticket_offers (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ticket_type ticket_type NOT NULL,
    tier SMALLINT NOT NULL CHECK (tier >= 1),
    zone ticket_zone NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    price_in_cents INTEGER NOT NULL CHECK (price_in_cents > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
