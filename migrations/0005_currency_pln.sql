-- Rename price column from cents to PLN
ALTER TABLE tasks RENAME COLUMN estimated_price_cents TO estimated_price_pln;
