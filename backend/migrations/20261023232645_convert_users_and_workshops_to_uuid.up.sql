-- │ └╴  20261023232645_convert_users_and_workshops_to_uuid.up.sql
-- no-transaction
PRAGMA foreign_keys = OFF;

-- Convert core_users.user_id from INTEGER to BLOB(16) UUID.
-- All other UUID columns (workshops.id, workshop_registrations.workshop_id)
-- are ALREADY BLOB from earlier migrations. Only core_users needs rebuilding.
--
-- Strategy:
--   1. Rebuild core_users with BLOB(16) PK, generating randomblob(16) for
--      each of the 23 existing rows.
--   2. Rebuild every FK-bearing table that references core_users.user_id
--      so their FK is BLOB-consistent.
--
-- Since we're on a fresh DB (23 users, 0 sessions, 0 resets, 0 orders,
-- 0 certificates, 4 registrations with user_id=NULL), the FK tables are
-- empty. Rebuild via SELECT * — no column mapping needed.
-- ── 1. Rebuild core_users with BLOB PK ───────────────────
CREATE TABLE core_users_new (
  user_id BLOB PRIMARY KEY,
  user_first_name TEXT,
  user_last_name TEXT,
  user_mobile TEXT,
  user_dob TEXT,
  user_email TEXT UNIQUE,
  user_verified INTEGER NOT NULL DEFAULT 0,
  user_password TEXT,
  user_picture TEXT,
  user_login_method TEXT,
  user_verification_token TEXT,
  user_token_expires_at TEXT,
  user_role TEXT NOT NULL DEFAULT 'user' CHECK (user_role IN ('admin', 'user')),
  user_created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  user_updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO
  core_users_new (
    user_id,
    user_first_name,
    user_last_name,
    user_mobile,
    user_dob,
    user_email,
    user_verified,
    user_password,
    user_picture,
    user_login_method,
    user_verification_token,
    user_token_expires_at,
    user_role,
    user_created_at,
    user_updated_at
  )
SELECT
  randomblob (16), -- 16-byte UUID-compatible value
  user_first_name,
  user_last_name,
  user_mobile,
  user_dob,
  user_email,
  user_verified,
  user_password,
  user_picture,
  user_login_method,
  user_verification_token,
  user_token_expires_at,
  user_role,
  COALESCE(user_created_at, CURRENT_TIMESTAMP),
  COALESCE(user_updated_at, CURRENT_TIMESTAMP)
FROM
  core_users;

DROP TABLE core_users;

ALTER TABLE core_users_new
RENAME TO core_users;

-- ── 2. Rebuild FK-bearing tables (all empty) ─────────────
-- Note: workshop_registrations.user_id is already BLOB from File A,
-- but its FK references core_users(user_id). Since we rebuilt core_users,
-- the FK target is now BLOB — the FK on registrations is already BLOB,
-- so no rebuild of workshop_registrations is needed. ✅
--
-- However, user_sessions, password_resets, orders, certificates were
-- created in File B with BLOB user_id FK referencing core_users. Since
-- core_users is now BLOB, those FKs are valid. All tables are empty,
-- so no data migration needed. No rebuild required. ✅
--
-- This is the payoff of doing BLOB from the start: one rebuild, not five.
PRAGMA foreign_keys = ON;
