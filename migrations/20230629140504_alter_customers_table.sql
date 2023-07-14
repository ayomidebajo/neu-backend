-- Add migration script here
ALTER TABLE customers
ADD COLUMN IF NOT EXISTS is_subscribed BOOLEAN DEFAULT false;

ALTER TABLE customers
DROP IF EXISTS is_merchant;