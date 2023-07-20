-- Add migration script here
-- CREATE TYPE identity_proof_type AS ENUM ('PASSPORT', 'NATIONAL_ID', 'DRIVERS_LICENSE', 'OTHERS');

CREATE TABLE IF NOT EXISTS kyc (
	id UUID NOT NULL PRIMARY KEY,
	merchant_id UUID NOT NULL,
	id_type identity_proof_type,
	id_number TEXT,
	id_image TEXT,
	business_id TEXT,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	CONSTRAINT unique_merchant_id_fkey FOREIGN KEY (merchant_id) REFERENCES merchants (id)
);