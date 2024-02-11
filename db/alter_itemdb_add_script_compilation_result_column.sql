ALTER TABLE ragnarok.item_db ADD COLUMN IF NOT EXISTS script_compilation bytea;
ALTER TABLE ragnarok.item_db ADD COLUMN IF NOT EXISTS script_compilation_hash bytea;