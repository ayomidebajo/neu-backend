-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE subscriptions(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   subscribed_at timestamptz NOT NULL
);


-- Create Customers Table
CREATE TABLE customers(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   fname TEXT NOT NULL,
   lname TEXT NOT NULL,
   phone_no TEXT,
   password TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   is_merchant BOOLEAN NOT NULL,
   is_verified BOOLEAN NOT NULL
);


