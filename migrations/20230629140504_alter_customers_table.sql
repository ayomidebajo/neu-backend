-- Add migration script here
ALTER TABLE customers
ADD COLUMN is_subscribed BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE customers
DROP is_merchant;