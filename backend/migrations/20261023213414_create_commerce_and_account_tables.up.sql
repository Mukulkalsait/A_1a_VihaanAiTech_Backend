-- │ ├╴  20261023213414_create_commerce_and_account_tables.up.sql
-- Migration: create commerce + account tables.
--
-- UUID FKs (user_id, workshop_id) are BLOB(16), matching core_users.user_id
-- and workshops.id. All internal counters (product_id, order_id, ...) stay
-- INTEGER AUTOINCREMENT.
-- ── 1. user_sessions ─────────────────────────────────────
CREATE TABLE IF NOT EXISTS user_sessions (
  session_id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id BLOB NOT NULL REFERENCES core_users (user_id) ON DELETE CASCADE,
  token_hash TEXT NOT NULL UNIQUE,
  user_agent TEXT,
  ip_address TEXT,
  expires_at TEXT NOT NULL,
  revoked_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_user_sessions_user ON user_sessions (user_id);

CREATE INDEX IF NOT EXISTS idx_user_sessions_expires ON user_sessions (expires_at);

-- ── 2. password_resets ───────────────────────────────────
CREATE TABLE IF NOT EXISTS password_resets (
  reset_id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id BLOB NOT NULL REFERENCES core_users (user_id) ON DELETE CASCADE,
  token_hash TEXT NOT NULL UNIQUE,
  expires_at TEXT NOT NULL,
  used_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_password_resets_user ON password_resets (user_id);

-- ── 3. products ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS products (
  product_id INTEGER PRIMARY KEY AUTOINCREMENT,
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT,
  price REAL NOT NULL DEFAULT 0.00,
  currency TEXT NOT NULL DEFAULT 'INR',
  billing_period TEXT NOT NULL DEFAULT 'one_time' CHECK (
    billing_period IN ('one_time', 'monthly', 'yearly')
  ),
  active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_products_active ON products (active);

-- ── 4. product_features ──────────────────────────────────
CREATE TABLE IF NOT EXISTS product_features (
  feature_id INTEGER PRIMARY KEY AUTOINCREMENT,
  product_id INTEGER NOT NULL REFERENCES products (product_id) ON DELETE CASCADE,
  feature_text TEXT NOT NULL,
  display_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_product_features_product ON product_features (product_id, display_order);

-- ── 5. orders ────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS orders (
  order_id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id BLOB REFERENCES core_users (user_id) ON DELETE SET NULL,
  subtotal REAL NOT NULL DEFAULT 0.00,
  discount REAL NOT NULL DEFAULT 0.00,
  total REAL NOT NULL DEFAULT 0.00,
  currency TEXT NOT NULL DEFAULT 'INR',
  status TEXT NOT NULL DEFAULT 'pending' CHECK (
    status IN (
      'pending',
      'paid',
      'failed',
      'cancelled',
      'refunded'
    )
  ),
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_orders_user ON orders (user_id);

CREATE INDEX IF NOT EXISTS idx_orders_status ON orders (status);

CREATE INDEX IF NOT EXISTS idx_orders_created ON orders (created_at);

-- ── 6. order_items ───────────────────────────────────────
-- item_id stays INTEGER: for 'product' it points to products.product_id (int).
-- For 'workshop' it would need to point to workshops.id (BLOB) — see TODO.
CREATE TABLE IF NOT EXISTS order_items (
  order_item_id INTEGER PRIMARY KEY AUTOINCREMENT,
  order_id INTEGER NOT NULL REFERENCES orders (order_id) ON DELETE CASCADE,
  item_type TEXT NOT NULL CHECK (item_type IN ('product', 'workshop')),
  item_id INTEGER NOT NULL, -- see TODO; workshop case not supported yet
  quantity INTEGER NOT NULL DEFAULT 1,
  unit_price REAL NOT NULL,
  line_total REAL NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_order_items_order ON order_items (order_id);

CREATE INDEX IF NOT EXISTS idx_order_items_item ON order_items (item_type, item_id);

-- ── 7. payments ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS payments (
  payment_id INTEGER PRIMARY KEY AUTOINCREMENT,
  order_id INTEGER NOT NULL REFERENCES orders (order_id) ON DELETE CASCADE,
  method TEXT NOT NULL CHECK (
    method IN ('screenshot', 'razorpay', 'stripe', 'manual')
  ),
  provider_payment_id TEXT,
  amount REAL NOT NULL,
  currency TEXT NOT NULL DEFAULT 'INR',
  status TEXT NOT NULL DEFAULT 'pending' CHECK (
    status IN ('pending', 'success', 'failed', 'refunded')
  ),
  screenshot_path TEXT,
  failure_reason TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_payments_order ON payments (order_id);

CREATE INDEX IF NOT EXISTS idx_payments_status ON payments (status);

CREATE INDEX IF NOT EXISTS idx_payments_provider ON payments (provider_payment_id);

-- ── 8. certificates ──────────────────────────────────────
CREATE TABLE IF NOT EXISTS certificates (
  certificate_id INTEGER PRIMARY KEY AUTOINCREMENT,
  certificate_number TEXT NOT NULL UNIQUE,
  user_id BLOB REFERENCES core_users (user_id) ON DELETE SET NULL,
  registration_id INTEGER REFERENCES workshop_registrations (registration_id) ON DELETE SET NULL,
  workshop_id BLOB REFERENCES workshops (id) ON DELETE SET NULL,
  pdf_path TEXT,
  issued_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_certificates_user ON certificates (user_id);

CREATE INDEX IF NOT EXISTS idx_certificates_registration ON certificates (registration_id);

-- ── 9. email_log ─────────────────────────────────────────
CREATE TABLE IF NOT EXISTS email_log (
  email_id INTEGER PRIMARY KEY AUTOINCREMENT,
  to_email TEXT NOT NULL,
  subject TEXT NOT NULL,
  template TEXT,
  status TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued', 'sent', 'failed')),
  error_message TEXT,
  related_type TEXT,
  related_id INTEGER,
  sent_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_email_log_to ON email_log (to_email);

CREATE INDEX IF NOT EXISTS idx_email_log_status ON email_log (status);

CREATE INDEX IF NOT EXISTS idx_email_log_related ON email_log (related_type, related_id);

-- ── 10. contact_messages ─────────────────────────────────
CREATE TABLE IF NOT EXISTS contact_messages (
  message_id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  phone_number TEXT,
  subject TEXT,
  message TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'new' CHECK (status IN ('new', 'read', 'replied', 'archived')),
  replied_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_contact_messages_status ON contact_messages (status);

CREATE INDEX IF NOT EXISTS idx_contact_messages_created ON contact_messages (created_at);

-- TODO: when orders flow is built, convert order_items to polymorphic BLOB:
--   item_id BLOB, stores either products.product_id as 8-byte blob or
--   workshops.id (16-byte) directly. Decide format then.
