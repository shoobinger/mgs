#!/usr/bin/env bash

cat ./sql/clean.sql ./sql/schema.sql ./sql/data.sql | sqlite3 mgs.sqlite
