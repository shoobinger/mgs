#!/usr/bin/env bash

cat ./sql/reset.sql ./sql/schema.sql ./sql/data.sql | sqlite3 mgs.sqlite
