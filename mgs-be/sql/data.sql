INSERT INTO assets (id, user_id, type, name, description)
VALUES (1, 1, 2, "USD", "US Dollar"),
       (2, 1, 3, "FXIT", "Index fund following major US IT companies"),
       (3, 1, 4, "BTC", "Bitcoin cryptocurrency"),
       (4, 1, 1, "Land", "");

INSERT INTO symbols (id, name, quoted_currency_id)
VALUES (1, "USDRUB", 2),
       (2, "FXIT", 2),
       (3, "BTCRUB", 2),
       (4, "BTCUSD", 1);

INSERT INTO asset_market_info (asset_id, display_symbol_id)
VALUES (1, 1),
       (2, 2),
       (3, 3);

INSERT INTO storages (id, name, type, user_id)
VALUES (1, "Tinkoff Invest", 2, 1),
       (2, "Openbank", 1, 1),
       (3, "Electrum wallet", 5, 1);

INSERT INTO asset_parts (id, asset_id, storage_id, quantity)
VALUES (1, 1, 1, 10000),
       (2, 1, 2, 20000),
       (3, 2, 1, 4000),
       (4, 3, 3, 100);

INSERT INTO asset_symbols (asset_id, symbol_id)
VALUES (1, 1),
       (2, 2),
       (3, 3),
       (3, 4);

INSERT INTO quotes (symbol_id, price)
VALUES (1, 7500),
       (2, 546900),
       (3, 43500000),
       (4, 572200);
