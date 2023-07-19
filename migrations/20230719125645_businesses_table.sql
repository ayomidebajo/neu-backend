-- Add migration script here
CREATE TABLE IF NOT EXISTS businesses (
	id UUID NOT NULL PRIMARY KEY,
	business_slug TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL UNIQUE
)