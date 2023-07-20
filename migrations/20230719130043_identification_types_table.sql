-- Add migration script here
CREATE TABLE IF NOT EXISTS identification (
	id SERIAL PRIMARY KEY,
	identification_slug identity_proof_type NOT NULL UNIQUE,
	name TEXT NOT NULL UNIQUE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);