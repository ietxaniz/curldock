# CurlDock Architecture Documentation

## Overview

CurlDock is a web application designed to simplify API testing and curl script management. It features a Rust backend and a React frontend. The application uses a reverse proxy approach to manage different types of requests and supports separate development and production modes.

## Components

### 1. Rust Backend
- Manages all endpoints
- Implements API logic
- Serves the Single Page Application (SPA) in production mode
- Acts as a reverse proxy for different types of requests

### 2. React Frontend
- Single Page Application (SPA)
- Served by Rust in production
- Served by Vite (or similar) in development mode

### 3. Static Assets
- Stored in a dedicated folder
- Served directly by Rust in production
- Served by development server in development mode

## Module Structure

```
front/
rest-examples/
src/
├── main.rs
├── config.rs
├── revproxy/
│   ├── mod.rs
│   └── routes.rs
|       |──handlers/
├── api/
│   ├── mod.rs
│   ├── routes.rs
│   └── handlers/
│       ├── mod.rs
│       ├── hello.rs
│       └── ... (other handlers)
```

## Request Flow

1. All requests go through the reverse proxy module first.
2. The reverse proxy determines the type of request:
   - API requests (/api/*) are routed to the API handlers
   - In development, non-API requests are redirected to the development server
   - In production, non-API requests are served from the static files or SPA

## Development vs Production

### Development Mode
- API requests handled by Rust backend
- Frontend and static files served by development server (e.g., Vite)
- Enables hot-reloading for frontend development

### Production Mode
- All requests handled by Rust backend
- Frontend served as pre-built static files
- Static assets served directly by Rust

## API Response Structure

For consistency, API responses follow this structure:

```json
{
  "status": "success" | "error",
  "message": "Human-readable message",
  "data": {
    // Endpoint-specific data (for successful responses)
  }
}
```

Error responses may include additional error details in place of the `data` field.

## Configuration

Environment-specific configuration (e.g., development server URL, production static file path) is managed through a central configuration module, allowing easy switching between development and production modes.

## Key Features

1. API Testing: Provides a user-friendly interface for creating, managing, and executing curl scripts for testing REST APIs.
2. Script Management: Stores curl scripts in a version-controlled manner, enabling easy sharing and collaboration.
3. Curl Command Conversion: Converts curl commands into a structured, user-friendly UI, making it easier to understand and modify complex curl commands.
4. Docker Integration: Leverages Docker to encapsulate the application and its dependencies, ensuring a consistent environment across different systems.

## Future Considerations

- Implement user authentication and authorization for secure access to API testing features.
- Develop a plugin system to extend functionality and support various API protocols.
- Integrate with popular version control systems for better script management.
- Implement real-time collaboration features for team-based API testing scenarios.
