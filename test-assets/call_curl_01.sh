#!/bin/sh
  curl \
  -v \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, World!"}' \
  --cookie "auth=examplecookie" \
  --write-out '\nNamelookup: %{time_namelookup}\nConnect: %{time_connect}\nAppconnect: %{time_appconnect}\nPretransfer: %{time_pretransfer}\nStarttransfer: %{time_starttransfer}\nTotal: %{time_total}' \
  http://localhost:2080/test/reversingpost