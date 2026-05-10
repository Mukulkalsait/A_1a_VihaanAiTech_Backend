-- Add migration script here
-- SQLite has limited ALTER TABLE support
PRAGMA foreign_keys=off;

CREATE TABLE core_users_new (
    user_id INTEGER PRIMARY KEY,

    user_first_name TEXT,
    user_last_name TEXT,
    user_mobile TEXT,
    user_dob TEXT,

    user_email TEXT NOT NULL UNIQUE,
    user_verified INTEGER NOT NULL DEFAULT 0,

    user_password TEXT,
    user_picture TEXT,
    user_login_method TEXT,

    user_verification_token TEXT,
    user_token_expires_at TEXT,
    user_role TEXT NOT NULL DEFAULT 'user' CHECK (user_role IN ('admin', 'user')),

    user_created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    user_updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO core_users_new SELECT * FROM core_users;
DROP TABLE core_users;
ALTER TABLE core_users_new RENAME TO core_users;

PRAGMA foreign_keys=on;
