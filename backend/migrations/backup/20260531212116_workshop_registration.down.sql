-- Add down migration script here

-- Drop triggers first
DROP TRIGGER IF EXISTS update_workshop_registrations_updated_at;

-- Drop indexes
DROP INDEX IF EXISTS idx_workshop_registrations_created_at;
DROP INDEX IF EXISTS idx_workshop_registrations_registration_status;
DROP INDEX IF EXISTS idx_workshop_registrations_payment_status;
DROP INDEX IF EXISTS idx_workshop_registrations_user_id;

-- Drop the table
DROP TABLE IF EXISTS workshop_registrations;

