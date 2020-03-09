INSERT INTO assets (id, user_id, type, name, description, last_snapshot_id)
VALUES (1, 1, 1, "USD", "Deposit", 2),
       (2, 1, 2, "FXIT", "US IT companies", NULL),
       (3, 1, 3, "BTC", NULL, NULL);

INSERT INTO asset_snapshots (id, asset_id, quantity, date)
VALUES (1, 1, 100, "2020-02-01"),
       (2, 1, 110, "2020-02-02");

INSERT INTO snapshot_values (id, snapshot_id, currency_id, value)
VALUES (1, 1, 1, 10000),
       (2, 1, 2, 630000),
       (3, 2, 1, 11000),
       (4, 2, 2, 691945);

