
DROP TABLE IF EXISTS ping;
DROP TABLE IF EXISTS computers;

CREATE TABLE computers (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    version     INTEGER,
    world       INTEGER,
    dimension   INTEGER,
    pos_x       INTEGER,
    pos_z       INTEGER,

    UNIQUE (world, dimension, pos_x, pos_z) -- There can be only one computer per chunk
);

CREATE TABLE ping (
    computer    INTEGER,
    -- Store the time as milliseconds since unix epoch
    time_hi     INTEGER,    -- Since sqlite cannot store 128bit integers, we store the higher bits
    time_lo     INTEGER,    -- separately from the lower bits

    PRIMARY KEY (computer, time_hi, time_lo),
    FOREIGN KEY (computer) REFERENCES computers(id)
);