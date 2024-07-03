# ScriptManager Design

## Overview
ScriptManager is a module designed to manage shell scripts stored in a specified directory. It provides thread-safe operations for listing, retrieving, creating, updating, and executing scripts.

## Structure

The ScriptManager module is organized as follows:

```
src/script_manager/
├── mod.rs           # Module entry point, re-exports public items
├── models.rs        # Defines data structures like ScriptInfo
├── manager.rs       # Implements the ScriptManager struct
├── operations/      # Individual operation implementations
│   ├── mod.rs
│   ├── list_scripts.rs
│   ├── get_script.rs
│   ├── create_script.rs
│   ├── update_script.rs
│   ├── delete_script.rs
│   └── execute_script.rs
├── design.md        # This design document
```

Each operation of the ScriptManager is implemented in its own file within the `operations/` directory. This structure promotes maintainability and makes it easier to add or modify individual operations.

## Concurrency and Thread Safety

The ScriptManager is designed to be thread-safe and allow controlled concurrent access:

1. The ScriptManager is wrapped in an `Arc` (Atomic Reference Counting) when created, allowing it to be safely shared across multiple threads.

2. A `Mutex` is used to control write operations, ensuring that only one write operation can occur at a time.

3. Read operations (like listing scripts) can occur concurrently without acquiring the lock.

4. Execute operations like curl calls can also occur concurrently.

5. Write operations (like creating or updating scripts) must acquire the lock before proceeding.

This design allows for:
- Safe sharing of a single ScriptManager instance across all API requests
- Concurrent read operations for improved performance
- Concurrent execution of scripts
- Serialized write operations to prevent race conditions

## Error Handling

Operations in ScriptManager return Results, allowing for clear error propagation. A custom error type (e.g., `ScriptManagerError`) will be defined to encompass various error scenarios (file I/O errors, script not found, etc.).
