CREATE TABLE asset_types (
    id INTEGER(8) PRIMARY KEY NOT NULL,
    code VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL
);

INSERT INTO asset_types (id, code, description)
VALUES (1, "CURRENCY", "Currencies"),
       (2, "ETF", "Exchange-traded funds"),
       (3, "CRYPTO", "Crypto-currencies");

CREATE TABLE currencies (
    id INTEGER(2) PRIMARY KEY NOT NULL,
    name VARCHAR(255)
);

INSERT INTO currencies (id, name)
VALUES (1, 'USD'),
       (2, 'RUB');

CREATE TABLE assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    type INTEGER(2),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE current_asset_values (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id INTEGER NOT NULL,
    currency_id INTEGER(2) NOT NULL,
    quantity INTEGER(8) NOT NULL,
    value INTEGER(8) NOT NULL
);

CREATE TABLE asset_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id INTEGER NOT NULL,
    quantity INTEGER(8) NOT NULL DEFAULT 0,
    date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE asset_snapshot_values (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    snapshot_id INTEGER NOT NULL,
    currency_id INTEGER(2) NOT NULL,
    value INTEGER(8) NOT NULL
);

CREATE TABLE asset_states (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id INTEGER NOT NULL,
    quantity INTEGER(8) NOT NULL,
    -- storage_id
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
