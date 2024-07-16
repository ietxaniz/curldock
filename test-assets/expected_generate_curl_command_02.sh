#!/bin/sh
  curl \
  -v \
  --write-out '\nNamelookup: %{time_namelookup}\nConnect: %{time_connect}\nAppconnect: %{time_appconnect}\nPretransfer: %{time_pretransfer}\nStarttransfer: %{time_starttransfer}\nTotal: %{time_total}' \
  http://localhost:2080/
# storebody data.parameter1 parameter1 ./test-assets/tmp/data.json
# storecookie auth auth ./test-assets/tmp/data.json
# loaddata ./test-assets/data.json parameter1 AUTH