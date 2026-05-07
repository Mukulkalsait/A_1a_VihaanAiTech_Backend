  1. root folder containn .env file, mentioning "DATABASE_URL" var.
  2. this decides the db location.
  3. even do we can ran the create user query from rust we dont do it. 
      - compile time data creation, 
      - but before compili time clippy check db and give errors, 
      - no compilition happens. 

# Database Migration Management

## Why migrations exist

As backend grows, database schema changes frequently:

- new columns
- new tables
- indexes
- constraints
- relations

---

# Migration Workflow

## 1. `.env` contains database location

```env
DATABASE_URL=sqlite://src/db/database/dev.db
```

This tells SQLx where DB exists.

---

# 2. Create migration file

```bash
sqlx migrate add create-core-users
```

Creates:

```text
migrations/
└── timestamp_create-core-users.sql
```

Write SQL schema inside that file.

Example:

```sql
CREATE TABLE core_users(
    user_id INTEGER PRIMARY KEY,
    user_email TEXT NOT NULL
);
```

---

# 3. Run migration

Run from project root where `migrations/` exists.

```bash
sqlx migrate run
```
SQLx checks:
- which migrations already ran
- which are pending
- Only new migrations execute.
---

# Production-safe schema updates

Suppose app is live and data already exists. Need new column:

```sql
user_bio TEXT
```

DO NOT:
- delete DB
- edit old migration
- recreate table

DO:
```bash
sqlx migrate add add-user-bio
```
Inside new migration:
```sql
ALTER TABLE core_users
ADD COLUMN user_bio TEXT;
```
Then run:
```bash
sqlx migrate run
```

Result:

- old data preserved
- schema updated safely
- only new migration runs

---

# Important Rule
Once migration runs in production:
NEVER modify old migration files.
Always create new migration files for schema changes.
Migrations are permanent database history.
