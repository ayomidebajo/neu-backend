-- Add migration script here
CREATE TABLE IF NOT EXISTS customers_app_settings (
	customer_id UUID NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	receive_email_notifications BOOLEAN NOT NULL DEFAULT FALSE,
	receive_sms_notifications BOOLEAN NOT NULL DEFAULT FALSE,
	receive_push_notifications BOOLEAN NOT NULL DEFAULT FALSE,
	CONSTRAINT unique_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES customers (id)
	);