-- Add up migration script here
-- migrations/20261026000000_extend_workshops_for_page_content.up.sql
-- no-transaction   (ALTER TABLE ADD COLUMN doesn't need transaction wrap, but keep safe)
ALTER TABLE workshops
ADD COLUMN time_label TEXT;

ALTER TABLE workshops
ADD COLUMN timezone TEXT;

ALTER TABLE workshops
ADD COLUMN whatsapp_group_url TEXT;

ALTER TABLE workshops
ADD COLUMN whatsapp_qr_image_path TEXT;

ALTER TABLE workshops
ADD COLUMN payment_qr_image_path TEXT;

ALTER TABLE workshops
ADD COLUMN support_phone TEXT;

ALTER TABLE workshops
ADD COLUMN support_whatsapp_url TEXT;

ALTER TABLE workshops
ADD COLUMN terms_url TEXT;

ALTER TABLE workshops
ADD COLUMN landing_tagline TEXT;

-- Populate the existing workshop with the current hardcoded values
UPDATE workshops
SET
  time_label = '11:00 AM to 12:00 PM',
  timezone = 'Asia/Kolkata',
  whatsapp_group_url = 'https://chat.whatsapp.com/J7mfgOtsOR5LABN4zwWUOH?s=cl&p=a&mlu=4&ilr=4',
  whatsapp_qr_image_path = '/images/app/WhGroupQR.jpeg',
  payment_qr_image_path = '/QRcode.jpeg',
  support_phone = '9834320594',
  support_whatsapp_url = 'https://wa.me/919834320594',
  terms_url = '/workshop-terms',
  landing_tagline = 'Less than the price of a dosa! 🥞🍽️'
WHERE
  slug = 'data-science-2026-06-07';

-- Trust badges (separate table)
CREATE TABLE IF NOT EXISTS workshop_badges (
  badge_id INTEGER PRIMARY KEY AUTOINCREMENT,
  workshop_id BLOB NOT NULL REFERENCES workshops (id) ON DELETE CASCADE,
  icon TEXT,
  text TEXT NOT NULL,
  display_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_workshop_badges_workshop ON workshop_badges (workshop_id, display_order);

-- Seed the two badges currently on the page
INSERT INTO
  workshop_badges (workshop_id, icon, text, display_order)
SELECT
  id,
  '🔒',
  'Your data is secure',
  1
FROM
  workshops
WHERE
  slug = 'data-science-2026-06-07';

INSERT INTO
  workshop_badges (workshop_id, icon, text, display_order)
SELECT
  id,
  '⭐⭐⭐⭐⭐',
  '4.8/5 with 1000+ satisfied parents',
  2
FROM
  workshops
WHERE
  slug = 'data-science-2026-06-07';
