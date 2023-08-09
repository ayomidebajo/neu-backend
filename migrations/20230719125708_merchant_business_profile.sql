-- Add migration script here
CREATE TABLE IF NOT EXISTS merchant_business_profile (
	id UUID NOT NULL PRIMARY KEY,
	merchant_id UUID NOT NULL,
	business_id UUID NOT NULL,
	no_of_employees INTEGER DEFAULT 0,
	services text[] DEFAULT '{}',
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	merchant_logo TEXT,
	business_description TEXT,
	city TEXT,
	state TEXT,
	country TEXT,
	postal_code INT,
	street_address TEXT,
	business_email TEXT,
	business_phone TEXT,
	business_website TEXT,
	business_social_media TEXT,
	CONSTRAINT unique_merchant_id_fkey FOREIGN KEY (merchant_id) REFERENCES merchants (id)
);