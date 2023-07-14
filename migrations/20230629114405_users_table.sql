-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   password VARCHAR NOT NULL,
   created_at timestamptz NOT NULL,
   is_merchant BOOLEAN NOT NULL,
   is_verified BOOLEAN NOT NULL
);