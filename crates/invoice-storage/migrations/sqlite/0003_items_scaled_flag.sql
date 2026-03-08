-- Adds a flag to track whether each invoice's item_json quantities have been rescaled to the new integer format

ALTER TABLE invoices ADD COLUMN items_scaled INTEGER NOT NULL DEFAULT 0;
