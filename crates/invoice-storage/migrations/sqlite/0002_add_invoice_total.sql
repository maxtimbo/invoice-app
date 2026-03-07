-- add cached total column to invoiced table
-- Stored as INTEGER (cents) to avoid floating point issues

ALTER TABLE invoices ADD COLUMN total INTEGER NOT NULL DEFAULT 0;
