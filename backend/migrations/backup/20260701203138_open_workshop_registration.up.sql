-- Add up migration script here

-- Create new migration for open workshop registrations
CREATE TABLE open_workshop_registrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    country_code TEXT DEFAULT '+91',
    payment_screenshot_path TEXT NOT NULL,
    workshop_name TEXT DEFAULT 'Data Science Workshop',
    workshop_date TEXT DEFAULT '2026-06-07',
    amount REAL DEFAULT 99.00,
    payment_status TEXT DEFAULT 'pending' CHECK (payment_status IN ('pending', 'verified', 'failed')),
    registered_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_open_workshop_email ON open_workshop_registrations(email);
CREATE INDEX idx_open_workshop_payment_status ON open_workshop_registrations(payment_status);
