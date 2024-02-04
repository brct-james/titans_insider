-- Your SQL goes here
CREATE TABLE IF NOT EXISTS item_hist (
    uuid            TEXT PRIMARY KEY,
    item_id         INTEGER NOT NULL,
    t_type          TEXT NOT NULL,
    uid             TEXT NOT NULL,
    tag1            TEXT,
    tag2            TEXT,
    tag3            TEXT,
    gold_qty        INTEGER NOT NULL DEFAULT 0,
    gems_qty        INTEGER NOT NULL DEFAULT 0,
    created         TEXT,
    tier            INTEGER,
    item_order      INTEGER,
    city_id         INTEGER,
    gold_price      INTEGER NOT NULL DEFAULT 0,
    gems_price      INTEGER NOT NULL DEFAULT 0,
    request_cycle   INTEGER NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    db_timestamp    BIGINT NOT NULL,
    CONSTRAINT uniqueness UNIQUE NULLS NOT DISTINCT (uid, request_cycle, t_type, tag1, tag2, tag3)
);
CREATE INDEX ON item_hist (uid);
CREATE INDEX ON item_hist (request_cycle);
CREATE INDEX ON item_hist (t_type);
CREATE INDEX ON item_hist (tag1);
CREATE INDEX ON item_hist (tag2);
CREATE INDEX ON item_hist (tag3);
CREATE INDEX ON item_hist (tier);
CREATE INDEX ON item_hist (db_timestamp);