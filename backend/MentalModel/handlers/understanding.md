1. First → **what is `pool` (very important)**
2. Then → how a **handler actually works (mentally)**
3. Then → your `create_user` flow (line-by-line)
4. Then → why structs look “different”
5. Then → `/me` and `/auth/google` (what they *actually do*)

---

# 🧠 1. What is `pool` (core concept)

You asked:

> what exactly pool is doing here?

### Think like this:

```text
Database = a restaurant
Connection = a table
Pool = waiter managing multiple tables
```

---

## Without pool ❌

Every time:

```text
request → open DB connection → query → close connection
```

👉 Slow + expensive

---

## With pool ✅

```text
App starts → creates pool (say 5 connections)

Request 1 → gets connection #1
Request 2 → gets connection #2
Request 3 → waits or uses free one
```

---

## In your code

```rust
let db = SqlitePoolOptions::new()
    .max_connections(6)
    .connect(&config.database_url)
    .await?;
```

👉 This creates:

```text
state.db = Pool of 6 DB connections
```

---

## Then inside handler

```rust
.execute(&state.db)
```

👉 Means:

```text
"hey pool, give me a connection → run query → return it"
```

---

# ⚡ KEY IDEA

You NEVER deal with raw DB connection.

You always use:

```rust
&state.db  // pool
```

---

# 🧠 2. What is a Handler (you’re confused here)

You said:

> i dont know what handlers are doing at all

Let’s simplify brutally:

---

## A handler = function that runs when a route is hit

```rust
.route("/users", post(create_user))
```

👉 means:

```text
HTTP POST /users → run create_user()
```

---

## Axum injects things into your function

```rust
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
)
```

This looks scary, but it's just:

---

### 1. `State(state)`

👉 Axum gives you:

```text
global app state (db + config)
```

---

### 2. `Json(payload)`

👉 Axum does:

```text
request body JSON → convert into CreateUser struct
```

---

## Example request

```json
POST /users

{
  "email": "test@gmail.com",
  "first_name": "Mukul",
  "last_name": "K",
  "password": "123"
}
```

---

## Axum converts it into:

```rust
CreateUser {
  email: "...",
  first_name: "...",
  last_name: "...",
  password: "..."
}
```

---

# 🧠 3. Now YOUR `create_user` (line by line)

```rust
pub async fn create_user(
```

👉 async because DB call takes time

---

```rust
State(state): State<AppState>,
```

👉 get access to:

```text
state.db
state.config
```

---

```rust
Json(payload): Json<CreateUser>,
```

👉 get request body as struct

---

```rust
let now = chrono::Utc::now().to_rfc3339();
```

👉 generate timestamp

---

```rust
sqlx::query!(
```

👉 compile-time checked SQL (very powerful)

---

```sql
INSERT INTO core_users (email,first_name,last_name,password_hash,created_at)
```

⚠️ PROBLEM:

Your DB now has:

```sql
user_email
user_first_name
user_password
```

👉 mismatch

---

```rust
payload.email
```

👉 comes from request JSON

---

```rust
.execute(&state.db)
```

👉 run query using pool

---

```rust
.await
```

👉 wait for DB to respond

---

```rust
.map_err(|_| ApiError::Internal)?;
```

👉 if error → return API error

---

```rust
Ok(Json(json!({"status":"user created"})))
```

👉 send response

---

# 🧠 4. Why structs look different?

You said:

> CreateUser != DB structure
> UserResponse also different

Good catch. This is intentional.

---

## There are 3 DIFFERENT models

### 1. Request model

```rust
CreateUser
```

👉 what client sends

---

### 2. DB model

```sql
core_users table
```

👉 actual storage

---

### 3. Response model

```rust
UserResponse
```

👉 what you return to client

---

## Why separate them?

Because:

```text
Client → should NOT control DB schema
DB → should NOT expose everything
```

---

## Example

DB:

```text
user_password
user_verification_token
```

❌ You NEVER return these

---

So:

```rust
UserResponse {
  id,
  email,
  first_name
}
```

✔ safe output

---



# ⚠️ BIG PROBLEM YOU STILL HAVE

Your DB schema now:

```sql
user_email
user_first_name
user_password
```

But your queries use:

```sql
email
first_name
password_hash
```

👉 This WILL break everything.

---

# ✅ What we should do next (slow + correct)

Now that you understand flow, next step is:

### 👉 Fix mismatch

Either:

**Option A (recommended):**
Rename DB columns to match handlers

OR

**Option B:**
Update all queries

---

Then we move to:

👉 rewrite `create_user` properly
👉 implement password hashing
👉 clean models

---

# 💬 Tell me next

Do you want to:

1. Fix DB + queries together
2. Rewrite `create_user` properly (step-by-step)
3. Understand `sqlx::query!` deeply
4. Build login flow from scratch

Say the number 👍
