-- Add migration script here
CREATE TABLE IF NOT EXISTS merchant_business_profile (
	id UUID NOT NULL PRIMARY KEY,
	merchant_id UUID NOT NULL,
	business_id UUID NOT NULL,
	no_of_employees INTEGER NOT NULL,
	services text[] DEFAULT '{}',
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	merchant_logo TEXT NOT NULL,
	business_description TEXT NOT NULL,
	business_address TEXT NOT NULL,
	business_email TEXT NOT NULL,
	business_phone TEXT NOT NULL,
	business_website TEXT NOT NULL,
	business_social_media TEXT NOT NULL
);