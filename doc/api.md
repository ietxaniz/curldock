# CurlDock API Design

## Base URL
All API endpoints are prefixed with `/api/v1`

## Endpoints

### Health Check
- GET /health
  - Returns the health status of the API

### Scripts
- GET /scripts[/{path}]
  - List all scripts in the specified path (or root if no path is provided)
  - Returns: 
    ```json
    { 
      "path": string, 
      "scripts": [
        { 
          "name": string, 
          "created_at": timestamp, 
          "updated_at": timestamp, 
          "is_folder": boolean 
        }
      ]
    }
    ```

- POST /script
  - Create a new curl script in the specified path (or root if no path is provided)
  - Body: 
    ```json
    {
      "name": "string",
      "path": "string",
      "curl_command": {
        "method": "string",
        "url": "string",
        "headers": [
          ["string", "string"]
        ],
        "data": "string or null",
        "options": {
          "verbose": "boolean",
          "insecure": "boolean"
        }
      }
    }
    ```
  - Returns: { "name": string, "created_at": timestamp }

- POST /script
  - Create a new curl script in the specified path (or root if no path is provided)
  - Body: 
    ```json
    {
      "name": "string",
      "path": "string",
      "curl_command": {
        "method": "string",
        "url": "string",
        "headers": [
          ["string", "string"]
        ],
        "data": "string or null",
        "options": {
          "verbose": "boolean",
          "insecure": "boolean"
        }
      }
    }
    ```
  - Returns: { "name": string, "created_at": timestamp }

- DELETE /scripts/{path}/{name}
  - Delete a script
  - Returns: { "success": boolean }


### Script-details

- **GET** `/script-details/{path}/{name}`
  - Get all information of a script
  - Returns:
    ```json
    {
      "name": "string",
      "path": "string",
      "curl_command": {
        "method": "string",
        "url": "string",
        "headers": [
          ["string", "string"]
        ],
        "data": "string or null",
        "options": {
          "verbose": "boolean",
          "insecure": "boolean"
        }
      },
      "created_at": "timestamp",
      "updated_at": "timestamp"
    }
    ```

### Execute

- POST /execute/{path}/{name}
  - Execute a specific script
  - Returns: { "name": string, "output": string, "executed_at": timestamp }


## Error Responses
All endpoints may return the following error responses:
- 400 Bad Request: { "error": "Invalid input", "details": string }
- 404 Not Found: { "error": "Resource not found" }
- 500 Internal Server Error: { "error": "Internal server error" }
