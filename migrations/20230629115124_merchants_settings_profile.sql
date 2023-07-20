-- Add migration script here
CREATE TABLE IF NOT EXISTS merchants_app_settings (
   id SERIAL PRIMARY KEY UNIQUE,
   merchant_id uuid,
   is_merchant BOOLEAN NOT NULL DEFAULT true,
   is_verified BOOLEAN DEFAULT false,
   business_name TEXT NOT NULL,
   business_logo TEXT,
   business_description TEXT,
   push_notification BOOLEAN DEFAULT false,
   email_notification BOOLEAN DEFAULT false,
   business_address TEXT ,
   slug TEXT,
   tags TEXT,
   no_of_employees INT DEFAULT 0,
   open_days TEXT,
   open_hours TEXT,
    updated_at timestamp with time zone,
    created_at timestamp with time zone NOT NULL,
    CONSTRAINT uk_merchant_id FOREIGN KEY (merchant_id) REFERENCES merchants(id)
);