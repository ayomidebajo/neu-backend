-- Add migration script here
CREATE TABLE IF NOT EXISTS payment_details_merchant (
	merchant_id UUID NOT NULL UNIQUE,
	business_id UUID NOT NULL,
	merchant_name TEXT NOT NULL,
	merchant_slug TEXT NOT NULL UNIQUE,
	merchant_email TEXT NOT NULL UNIQUE,
	merchant_phone TEXT NOT NULL UNIQUE,
	merchant_address TEXT NOT NULL,
	merchant_logo TEXT NOT NULL,
	merchant_account_no TEXT NOT NULL,
	merchant_bank_name TEXT NOT NULL,
	CONSTRAINT fk_merchant_id FOREIGN KEY (merchant_id) REFERENCES merchants(id),
	CONSTRAINT fk_business_id FOREIGN KEY (business_id) REFERENCES businesses(id),
	CONSTRAINT pk_merchant_details PRIMARY KEY (merchant_id, merchant_account_no),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);