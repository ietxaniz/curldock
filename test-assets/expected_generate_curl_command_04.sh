#!/bin/sh
  curl \
  -v \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"message": "Hello, World!", "message2": "akilote", "message3": "pain maker"}' \
  --cookie 'auth=examplecookie' \
  --cookie 'refresh=anothercookie' \
  --write-out '\nNamelookup: %{time_namelookup}\nConnect: %{time_connect}\nAppconnect: %{time_appconnect}\nPretransfer: %{time_pretransfer}\nStarttransfer: %{time_starttransfer}\nTotal: %{time_total}' \
  http://localhost:2080/test/reversingpost
# storebody message message ./test-assets/tmp/data.json
# storebody received.message2 message2 ./test-assets/tmp/data.json
# storebody received.message3 message3 ./test-assets/tmp/data-1.json
# storecookie auth auth ./test-assets/tmp/data.json
# loaddata ./test-assets/data.json parameter1 AUTH