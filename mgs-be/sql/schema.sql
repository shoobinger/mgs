CREATE TABLE asset_types (
    id INTEGER(8) PRIMARY KEY NOT NULL,
    code VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL
);

INSERT INTO asset_types (id, code, description)
VALUES (1, "OTHER", "Other"),
       (2, "CURRENCY", "Currencies"),
       (3, "ETF", "Exchange-traded funds"),
       (4, "CRYPTO", "Crypto-currencies");

CREATE TABLE currencies (
    id INTEGER(2) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

INSERT INTO currencies (id, name)
VALUES (1, 'USD'),
       (2, 'RUB');

CREATE TABLE storage_types (
    id INTEGER(2) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

INSERT INTO storage_types (id, name)
VALUES (1, 'Other'),
       (2, 'Bank account'),
       (3, 'Broker account'),
       (4, 'Exchange account'),
       (5, 'Cryptowallet'),
       (6, 'Cash');

CREATE TABLE storages (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    type INTEGER(2) NOT NULL,
    user_id INTEGER NOT NULL
);

CREATE TABLE symbols (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    quoted_currency_id INTEGER(2) NOT NULL
);

CREATE TABLE assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    type INTEGER(2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);


CREATE TABLE asset_parts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_id INTEGER NOT NULL,
    storage_id INTEGER NOT NULL,
    quantity INTEGER(8) NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE asset_parts_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    asset_part_id INTEGER NOT NULL,
    quantity INTEGER(8) NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE asset_market_info (
    asset_id INTEGER PRIMARY KEY NOT NULL,
    display_symbol_id INTEGER NOT NULL
);

CREATE TABLE asset_symbols (
    asset_id INTEGER NOT NULL,
    symbol_id INTEGER NOT NULL,
    PRIMARY KEY (asset_id, symbol_id)
);

CREATE TABLE prices (
    symbol_id INTEGER PRIMARY KEY NOT NULL,
    value INTEGER(8) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)

