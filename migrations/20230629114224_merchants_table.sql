-- Add migration script here
CREATE TABLE IF NOT EXISTS merchants(
   id uuid NOT NULL UNIQUE,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   business_name TEXT NOT NULL,
   password VARCHAR NOT NULL,
   created_at timestamp with time zone NOT NULL,
   updated_at timestamp with time zone,
   is_verified BOOLEAN DEFAULT false,
   is_active BOOLEAN DEFAULT false
);