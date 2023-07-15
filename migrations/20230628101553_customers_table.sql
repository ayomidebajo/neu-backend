-- Add migration script here
-- Create Customers Table
CREATE TABLE IF NOT EXISTS customers(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   password VARCHAR NOT NULL,
   created_at timestamp with time zone NOT NULL,
   updated_at timestamp with time zone,
   is_merchant BOOLEAN,
   is_verified BOOLEAN NOT NULL,
   is_subscribed BOOLEAN NOT NULL
);