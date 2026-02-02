```bash 

vihaanaigroup-backend/
│
├── backend/                 # Rust application
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── .env.example
│   │
│   └── src/
│       ├── main.rs          # App entry
│       ├── app.rs           # App builder
│       │
│       ├── config/          # Env & config
│       │   └── mod.rs
│       │
│       ├── db/              # DB pool & helpers
│       │   ├── mod.rs
│       │   └── pool.rs
│       │
│       ├── models/          # DB models (maps to tables)
│       │   ├── mod.rs
│       │   ├── user.rs
│       │   ├── app.rs
│       │   ├── purchase.rs
│       │   └── order.rs
│       │
│       ├── routes/          # Route definitions
│       │   ├── mod.rs
│       │   ├── auth.rs
│       │   ├── users.rs
│       │   ├── apps.rs
│       │   └── purchases.rs
│       │
│       ├── handlers/        # Request handlers
│       │   ├── mod.rs
│       │   ├── auth.rs
│       │   ├── users.rs
│       │   ├── apps.rs
│       │   └── purchases.rs
│       │
│       ├── services/        # Business logic
│       │   ├── mod.rs
│       │   ├── auth_service.rs
│       │   ├── user_service.rs
│       │   └── purchase_service.rs
│       │
│       ├── middleware/      # Auth, logging, etc.
│       │   ├── mod.rs
│       │   └── auth.rs
│       │
│       ├── utils/           # Helpers
│       │   ├── mod.rs
│       │   ├── jwt.rs
│       │   └── password.rs
│       │
│       └── errors.rs        # Unified error handling
│
├── infra/
│   ├── podman-compose.yml
│   ├── nginx/
│   │   └── default.conf
│   └── mysql/
│       └── init.sql
│
├── ci/
│   └── Jenkinsfile          # Later
│
└── README.md

```


# ORDER

```bash 
Config → Errors → DB Pool → App State → Routing → Handlers → Services → Models
```
