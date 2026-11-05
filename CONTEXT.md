# Toto Ticket

A ticketing platform where customers reserve and buy tickets from ticket offers.

## Language

**Customer**:
A person who reserves and buys tickets.
_Avoid_: User, client, buyer

**Ticket Offer**:
A stock of tickets of one ticket type, tier and zone, sold at one price.

**Reservation**:
A temporary hold of one or more tickets from a single Ticket Offer for a Customer, lasting 10 minutes unless converted into a purchase.
_Avoid_: Booking

### Reservation statuses

**Active**:
A Reservation whose 10 minutes have not run out and that was not purchased.
_Avoid_: Valid, pending

**Expired**:
A Reservation whose 10 minutes ran out before it was purchased; its tickets return to the Ticket Offer.

**Purchased**:
A Reservation the Customer paid for, so its tickets are theirs.
_Avoid_: Converted, confirmed
