-- Add migration script here
ALTER TABLE merchant_business_profile
	ALTER COLUMN business_website DROP NOT NULL;

ALTER TABLE merchant_business_profile
	ALTER COLUMN business_social_media DROP NOT NULL;

ALTER TABLE merchant_business_profile
	ALTER COLUMN business_description DROP NOT NULL;