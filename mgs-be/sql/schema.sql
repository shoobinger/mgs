CREATE TABLE asset_types (
    id INTEGER(8) PRIMARY KEY NOT NULL,
    code VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL
);

INSERT INTO asset_types (id, code, description)
VALUES (1, "CURRENCY", "Currencies"),
       (2, "ETF", "Exchange-traded funds"),
       (3, "CRYPTO", "Crypto-currencies");

CREATE TABLE assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    type INTEGER(8),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_snapshot_id INTEGER,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE asset_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE currencies (
    id INTEGER(2) PRIMARY KEY NOT NULL,
    name VARCHAR(255)
);

INSERT INTO currencies (id, name)
VALUES (1, 'USD'),
       (2, 'RUB');

CREATE TABLE snapshot_values (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    snapshot_id INTEGER NOT NULL,
    currency_id INTEGER(2) NOT NULL,
    value INTEGER(8) NOT NULL
);
