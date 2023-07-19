-- Add migration script here
CREATE TABLE IF NOT EXISTS kyc (
	id UUID NOT NULL PRIMARY KEY,
	merchant_id UUID NOT NULL,
	id_type UUID NOT NULL,
	id_number TEXT NOT NULL,
	id_image TEXT NOT NULL,
	business_id TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);