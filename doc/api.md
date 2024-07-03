# CurlDock API Design

## Base URL
All API endpoints are prefixed with `/api/v1`

## Endpoints

### Health Check
- GET /health
  - Returns the health status of the API

### Scripts
- POST /scripts
  - Create a new curl script
  - Body: { "name": string, "content": string }
  - Returns: { "id": string, "name": string, "content": string, "created_at": timestamp }

- GET /scripts
  - List all scripts
  - Query parameters:
    - page: int (default 1)
    - per_page: int (default 20)
    - sort: string (e.g., "name", "-created_at")
    - filter: string (e.g., "name:contains:test")
  - Returns: { "total": int, "page": int, "per_page": int, "scripts": [{ "id": string, "name": string, "created_at": timestamp, "last_executed": timestamp, "execution_count": int }] }

- GET /scripts/{id}
  - Get details of a specific script
  - Returns: { "id": string, "name": string, "content": string, "created_at": timestamp, "last_executed": timestamp, "execution_count": int }

- PUT /scripts/{id}
  - Update an existing script
  - Body: { "name": string, "content": string }
  - Returns: { "id": string, "name": string, "content": string, "updated_at": timestamp }

- DELETE /scripts/{id}
  - Delete a script
  - Returns: { "success": boolean }

- POST /scripts/{id}/execute
  - Execute a specific script
  - Returns: { "id": string, "output": string, "executed_at": timestamp }

### Bulk Operations
- POST /bulk/scripts
  - Bulk create scripts
  - Body: [{ "name": string, "content": string }, ...]
  - Returns: [{ "id": string, "name": string, "created_at": timestamp }, ...]

- PUT /bulk/scripts
  - Bulk update scripts
  - Body: [{ "id": string, "name": string, "content": string }, ...]
  - Returns: [{ "id": string, "updated_at": timestamp }, ...]

- DELETE /bulk/scripts
  - Bulk delete scripts
  - Body: { "ids": [string] }
  - Returns: { "success": boolean, "deleted_count": int }

## Error Responses
All endpoints may return the following error responses:
- 400 Bad Request: { "error": "Invalid input", "details": string }
- 401 Unauthorized: { "error": "Authentication required" }
- 403 Forbidden: { "error": "Insufficient permissions" }
- 404 Not Found: { "error": "Resource not found" }
- 500 Internal Server Error: { "error": "Internal server error" }