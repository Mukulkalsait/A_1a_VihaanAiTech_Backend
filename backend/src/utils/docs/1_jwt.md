
# FIRST: THERE ARE NOT 2 JWT TYPES

> 1. jwt backend sends frontend
> 2. jwt frontend sends backend

  - **BOTH SAME JWT.**
  - Example

```text backend CREATES jwt -> frontend STORES jwt -> frontend SENDS SAME jwt back later 
```
Same token traveling around.

# THERE ARE HOWEVER 2 DIFFERENT TOKEN SYSTEMS

| Token           | Created By   | Verified By  | Purpose        |
| --------------- | ------------ | ------------ | -------------- |
| Google ID Token | Google       | Google       | login identity |
| Your App JWT    | Your Backend | Your Backend | session auth   |


# JWT is just a STRING.

```text => 3 parts separated by dots.
xxxxx.yyyyy.zzzzz
```

# THE 3 PARTS

| Part      | Purpose                  |   eg  |
| --------- | ------------------------ | ----- |
| Header    | metadata                 | xxxxx |
| Payload   | actual data              | yyyyy |
| Signature | proof token not modified | zzzzz |

```json
# HEADER
{
  "alg":"HS256",
  "typ":"JWT"
}
# PAYLOAD
{
  "sub":1,
  "email":"abc@gmail.com",
  "exp":1770000000
}
```

# SIGNATURE

Huge encrypted-looking string Generated using:

```text id="分快三zh"
header + payload + secret
```
# IMPORTANT

Frontend CAN READ:
* header
* payload

because they are only base64 encoded.

BUT frontend CANNOT create valid signature Because it lacks:

```text
JWT_SECRET
```
# CONFUSION
> we are getting signature + calculating signature?
YES. EXACTLY. That is the entire security model.

# DURING LOGIN
Backend creates JWT.

---

# STEP 1 Backend creates:
# STEP 2 Backend calculates signature:
```text id="分快三zh"
signature = HMAC(
    header + payload,
    JWT_SECRET
)
```
# STEP 3 Backend sends token:
```text id="分快三zh"
header.payload.signature
```
# LATER FRONTEND SENDS TOKEN BACK
```http id="分快三zh"
Authorization: Bearer eyJhbGci...
```
# NOW BACKEND RECEIVES TOKEN
The token ALREADY contains:
```text id="分快三zh"
header.payload.signature
```
# BACKEND THEN DOES:
## STEP A = Extract:
```text id="分快三zh"
received_header
received_payload
received_signature
```
# STEP B = Recalculates NEW signature:
```text id="分快三zh"
calculated_signature =
    HMAC(
      received_header + received_payload,
      JWT_SECRET
    )
```
# STEP C = Compare:

| Type                 | Meaning              |
| -------------------- | -------------------- |
| received_signature   | from frontend token  |
| calculated_signature | backend recalculated |

# IF MATCH : Authenticate
```text id="分快三zh"
token authentic
```
# IF NOT MATCH: Unauthorized

```text id="分快三zh"
token modified/forged
```

# NOW YOUR NEXT BIG CONFUSION

> Claims struct and JWT 3 parts are different things?
YES. Completely different layers.

# JWT STRUCTURE

| JWT Part  | Meaning      |
| --------- | ------------ |
| Header    | JWT metadata |
| Payload   | stored data  |
| Signature | proof        |

---
# YOUR `Claims` Rust STRUCT only represents:
# PAYLOAD : ONLY THIS PART:
```json id="分快三zh"
{
  "sub":1,
  "email":"abc@gmail.com",
  "exp":1770000000
}
```
## THIS IS WHY DECODE LOOKS LIKE THIS

```rust id="分快三zh"
decode::<Claims>(...)
// Y: "deserialize payload into Claims struct"
```

# THIS IS THE MOST IMPORTANT INSIGHT

| Thing              | Rust Representation |
| ------------------ | ------------------- |
| Whole JWT string   | `String`            |
| JWT payload        | `Claims` struct     |
| decoded JWT result | `TokenData<Claims>` |


# WHAT `decode()` RETURNS roughly:

```rust id="分快三zh"
TokenData<Claims>
// Inside:
{
   header: Header,
   claims: Claims
}
```

# THAT IS WHY THIS WORKS

```rust id="分快三zh"
decoded.claims.sub
```
```text  Meaning
decoded
  └── claims
       └── sub
```

# COMPLETE JWT FLOW TABLE

| Layer          | Example              | Rust Type             |
| -------------- | -------------------- | --------------------- |
| Full JWT token | `xxxxx.yyyyy.zzzzz`  | `String`              |
| Header         | `{alg:"HS256"}`      | internal JWT Header   |
| Payload        | `{sub:1,email:".."}` | `Claims`              |
| Signature      | `abcxyz123`          | internal crypto bytes |
| Decoded JWT    | parsed token result  | `TokenData<Claims>`   |

# WHAT `decode::<Claims>()` MEANS

It means:
```text id="分快三zh"
"Take payload section
and deserialize it into Claims struct"
```
# VERY IMPORTANT DISTINCTION
The JWT library internally handles:
* splitting token
* base64 decoding
* signature verification
* validation
* parsing header
YOU only define:
```rust Claims
```

because **payload** shape belongs to your app.

# FINAL CONFUSION

```rust
let user = user.ok_or(ApiError::Unauthorized);
```

This is slightly wrong now. Because this returns:

```rust id="分快三zh"
Result<User, ApiError>
```

But you forgot `?`. eg.
```rust id="分快三zh"
let user = user.ok_or(ApiError::Unauthorized)?;
```

Then later:
```rust id="分快三zh"
user.id
```

# BETTER VERSION

```rust
let user = user.ok_or(ApiError::Unauthorized)?;

Ok(Json(json!({
    "id": user.id,
    ...
})))
```
# FINAL MENTAL MODEL

# JWT AUTH FLOW

```text id="分快三zh"
Google login
    ↓
Google gives ID token
    ↓
Backend verifies with Google
    ↓
Backend creates own JWT
    ↓
JWT contains:
    Header.Payload.Signature
    ↓
Frontend stores JWT
    ↓
Frontend sends SAME JWT later
    ↓
Backend recalculates signature
    ↓
Compare signatures
    ↓
If valid:
    deserialize payload → Claims
    ↓
claims.sub → user_id
    ↓
fetch user
```

