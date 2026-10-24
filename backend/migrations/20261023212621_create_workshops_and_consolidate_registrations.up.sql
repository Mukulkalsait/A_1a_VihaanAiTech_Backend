-- Y:  file:  20261023212621_create_workshops_and_consolidate_registrations.up.sql
--
-- Migration: create workshops table + consolidate the two registration tables.
--
-- UUID storage: workshops.id is BLOB(16). The seed row uses a hard-coded
-- BLOB UUID so dev and prod get the SAME workshop id (important: existing
-- registrations in prod reference it, and we want identical data across envs).
--
-- The seed UUID is: 51dd4cfa-05e9-42e2-85e0-09a324b992f4
-- As a BLOB literal: X'51dd4cfa05e942e285e009a324b992f4'
-- ── 1. workshops table ────────────────────────────────────
CREATE TABLE IF NOT EXISTS workshops (
  id BLOB PRIMARY KEY,
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT,
  date TEXT NOT NULL,
  amount REAL NOT NULL DEFAULT 99.00,
  currency TEXT NOT NULL DEFAULT 'INR',
  capacity INTEGER,
  status TEXT NOT NULL DEFAULT 'draft',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_workshops_status ON workshops (status);

CREATE INDEX IF NOT EXISTS idx_workshops_date ON workshops (date);

-- ── 2. seed default workshop with fixed BLOB UUID ────────
INSERT
OR IGNORE INTO workshops (
  id,
  slug,
  name,
  description,
  date,
  amount,
  currency,
  capacity,
  status
)
VALUES
  (
    X'51dd4cfa05e942e285e009a324b992f4',
    'data-science-2026-06-07',
    'Data Science Workshop',
    'Hands-on introduction to data science fundamentals.',
    '2026-06-07',
    99.00,
    'INR',
    100,
    'open'
  );

-- ── 3. add workshop_id column to workshop_registrations ──
ALTER TABLE workshop_registrations
ADD COLUMN workshop_id BLOB REFERENCES workshops (id);

-- ── 4. copy legacy rows from open_workshop_registrations ──
INSERT INTO
  workshop_registrations (
    user_id,
    workshop_id,
    workshop_name,
    workshop_date,
    amount,
    payment_screenshot_path,
    payment_status,
    registration_status,
    attended,
    attended_at,
    created_at,
    updated_at,
    full_name,
    email,
    phone_number,
    country_code
  )
SELECT
  NULL,
  X'51dd4cfa05e942e285e009a324b992f4',
  workshop_name,
  workshop_date,
  amount,
  payment_screenshot_path,
  payment_status,
  'registered',
  0,
  NULL,
  registered_at,
  updated_at,
  full_name,
  email,
  phone_number,
  country_code
FROM
  open_workshop_registrations;

-- ── 5. drop legacy table ─────────────────────────────────
DROP TABLE open_workshop_registrations;
