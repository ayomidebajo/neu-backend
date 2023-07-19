-- Add migration script here
CREATE TABLE IF NOT EXISTS payment_details_merchant (
	merchant_id UUID NOT NULL PRIMARY KEY,
	business_id UUID NOT NULL,
	merchant_name TEXT NOT NULL,
	merchant_slug TEXT NOT NULL UNIQUE,
	merchant_email TEXT NOT NULL UNIQUE,
	merchant_phone TEXT NOT NULL UNIQUE,
	merchant_address TEXT NOT NULL,
	merchant_logo TEXT NOT NULL,
	merchant_account_no TEXT NOT NULL,
	merchant_bank_name TEXT NOT NULL
);