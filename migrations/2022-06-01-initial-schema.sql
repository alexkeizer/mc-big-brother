

DROP TABLE computer;
CREATE TABLE computer
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    chunk_x     INTEGER NOT NULL,
    chunk_y     INTEGER NOT NULL,
    is_online   BOOLEAN NOT NULL    DEFAULT 0,
    updated_at  INTEGER NOT NULL    DEFAULT (strftime('%s','now')),

    CHECK (is_online in (0,1))
);