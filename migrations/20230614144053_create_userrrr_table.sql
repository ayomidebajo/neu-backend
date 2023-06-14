-- Add migration script here
-- Create merchants Table
CREATE TABLE merchants(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   business_name TEXT NOT NULL,
   phone_no TEXT NULL,
   password TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   is_merchant BOOLEAN NOT NULL,
   is_verified BOOLEAN NOT NULL
);