#!/bin/sh

curl -v -X POST http://localhost:2080/api/v1/script \
-H "Content-Type: application/json" \
-d '{
  "name": "list_root.sh",
  "path": "",
  "curl_command": {
    "method": "GET",
    "url": "http://localhost:2080/api/v1/scripts",
    "headers": [],
    "data": null,
    "options": {
      "verbose": true,
      "insecure": false
    }
  }
}'
