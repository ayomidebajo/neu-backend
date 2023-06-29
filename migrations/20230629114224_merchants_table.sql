-- Add migration script here
CREATE TABLE merchants(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   business_name TEXT NOT NULL,
   password VARCHAR NOT NULL,
   created_at timestamptz NOT NULL,
   is_verified BOOLEAN NOT NULL,
   is_active BOOLEAN NOT NULL
);