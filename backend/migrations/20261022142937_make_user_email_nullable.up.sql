-- Add up migration script here


CREATE TABLE core_users_new (
    user_id INTEGER PRIMARY KEY,
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
    user_role TEXT NOT NULL DEFAULT 'user'
        CHECK (user_role IN ('admin', 'user')),
    user_created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    user_updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO core_users_new (
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
FROM core_users;

DROP TABLE core_users;

ALTER TABLE core_users_new RENAME TO core_users;

