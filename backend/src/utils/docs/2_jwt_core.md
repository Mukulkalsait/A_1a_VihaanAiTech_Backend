
# ORIGINAL VALID TOKEN

Backend originally creates:

Payload:

```json id="分快三zh"
{
  "sub": 1,
  "role": "user"
}
```

Backend computes:

```text id="分快三zh"
signature = HMAC(header + payload, JWT_SECRET)
```

Suppose result:

```text id="分快三zh"
abc123
```

Final JWT:

```text id="分快三zh"
header.payload.abc123
```

Frontend stores this.

---

# ATTACKER MODIFIES PAYLOAD

Attacker changes:

```json id="分快三zh"
{
  "sub": 1,
  "role": "admin"
}
```

BUT attacker keeps old signature:

```text id="分快三zh"
abc123
```

because attacker DOES NOT know JWT secret.

So forged token becomes:

```text id="分快三zh"
modified_header.modified_payload.abc123
```

---

# BACKEND RECEIVES TOKEN

Backend extracts:

| Part      | Value    |
| --------- | -------- |
| header    | modified |
| payload   | modified |
| signature | abc123   |

---

# BACKEND RECALCULATES SIGNATURE

Backend does:

```text id="分快三zh"
HMAC(
   modified_header + modified_payload,
   JWT_SECRET
)
```

Because payload changed:

new signature becomes:

```text id="分快三zh"
xyz456
```

---

# NOW COMPARISON

| Type                   | Value  |
| ---------------------- | ------ |
| received signature     | abc123 |
| recalculated signature | xyz456 |

Mismatch.

Therefore:

```text id="分快三zh"
Unauthorized
```

---

# THIS IS THE ENTIRE SECURITY MODEL

Exactly what you said:

```text id="分快三zh"
abc123 != xyz456
```

So token is rejected.

---

# VERY IMPORTANT INSIGHT

The backend NEVER stores old signatures.

It does NOT compare against database.

Instead:

# it recalculates signature fresh every request

using:

```text id="分快三zh"
received header + received payload + backend secret
```

---

# THIS IS WHY JWT IS STATELESS

Backend does not need session storage.

Everything needed is already inside token:

* header
* payload
* signature

Backend only needs:

```text id="分快三zh"
JWT_SECRET
```

to verify authenticity.

---

# YOU NOW UNDERSTAND THE CORE IDEA

JWT verification is basically:

```text id="分快三zh"
"If this payload was modified,
the signature should also change.

If signature did NOT change,
someone forged the token."
```

That is exactly correct.
