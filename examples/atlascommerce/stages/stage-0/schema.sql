CREATE TABLE legacy_exports(id bigint primary key, created_at timestamptz not null);
INSERT INTO legacy_exports(id, created_at) SELECT id, now() FROM export_queue WHERE processor = 'legacy-export';
