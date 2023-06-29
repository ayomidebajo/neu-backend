-- Add migration script here
CREATE TABLE merchants_settings_profile (
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   password VARCHAR NOT NULL,
   updated_at timestamptz NOT NULL,
   is_merchant BOOLEAN NOT NULL,
   is_verified BOOLEAN NOT NULL,
   business_name TEXT NOT NULL,
   business_logo TEXT NOT NULL,
   business_description TEXT NOT NULL,
   business_address TEXT NOT NULL,
   slug TEXT NOT NULL,
   tags TEXT NOT NULL,
   no_of_employees INT NOT NULL,
   open_days TEXT NOT NULL,
   open_hours TEXT NOT NULL
);