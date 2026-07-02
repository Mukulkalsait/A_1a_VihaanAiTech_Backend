-- Add up migration script here

-- Create workshop registrations table
CREATE TABLE IF NOT EXISTS workshop_registrations (
    registration_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    workshop_name TEXT NOT NULL DEFAULT 'Data Science Workshop',
    workshop_date TEXT NOT NULL,
    amount REAL NOT NULL DEFAULT 99.00,
    payment_screenshot_path TEXT,
    payment_status TEXT NOT NULL DEFAULT 'pending',
    registration_status TEXT NOT NULL DEFAULT 'registered',
    attended INTEGER NOT NULL DEFAULT 0,
    attended_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign key constraint
    FOREIGN KEY (user_id) REFERENCES core_users(user_id) ON DELETE CASCADE,
    
    -- Constraints
    CHECK (payment_status IN ('pending', 'verified', 'failed')),
    CHECK (registration_status IN ('registered', 'attended', 'cancelled', 'no_show')),
    
    -- Unique constraint to prevent duplicate registrations
    UNIQUE(user_id, workshop_name, workshop_date)
);

-- Create indexes for better query performance
CREATE INDEX idx_workshop_registrations_user_id ON workshop_registrations(user_id);
CREATE INDEX idx_workshop_registrations_payment_status ON workshop_registrations(payment_status);
CREATE INDEX idx_workshop_registrations_registration_status ON workshop_registrations(registration_status);
CREATE INDEX idx_workshop_registrations_created_at ON workshop_registrations(created_at);

-- Optional: Create a trigger to automatically update updated_at
CREATE TRIGGER IF NOT EXISTS update_workshop_registrations_updated_at 
AFTER UPDATE ON workshop_registrations
BEGIN
    UPDATE workshop_registrations 
    SET updated_at = CURRENT_TIMESTAMP 
    WHERE registration_id = NEW.registration_id;
END;
