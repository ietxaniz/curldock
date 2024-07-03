# CurlDock Curl Manager Module

The Curl Manager module is responsible for handling all curl-related operations in CurlDock.

## Functions

### create_script
- Input: name: String, content: String
- Output: Result<(), Error>
- Description: Creates a new curl script with the given name (path) and content.

### read_script
- Input: name: String
- Output: Result<Script, Error>
- Description: Retrieves the script with the given name (path).

### update_script
- Input: name: String, content: String
- Output: Result<(), Error>
- Description: Updates the script with the given name (path).

### delete_script
- Input: name: String
- Output: Result<(), Error>
- Description: Deletes the script with the given name (path).

### execute_script
- Input: name: String
- Output: Result<ExecutionResult, Error>
- Description: Executes the script with the given name (path) and returns the execution result.

### list_scripts
- Input: filter: Option<Filter>, sort: Option<Sort>, page: Option<Page>
- Output: Result<(Vec<ScriptMetadata>, Pagination), Error>
- Description: Lists scripts based on the provided filter, sort, and pagination parameters.

## Data Structures

### Script
- name: String (path of the script file)
- content: String
- created_at: DateTime
- updated_at: DateTime
- last_executed: Option<DateTime>
- execution_count: u32

### ScriptMetadata
- name: String (path of the script file)
- created_at: DateTime
- last_executed: Option<DateTime>
- execution_count: u32

### ExecutionResult
- output: String
- executed_at: DateTime

### Filter
- field: String
- operator: FilterOperator
- value: String

### FilterOperator
- Enum: Contains, Equals, GreaterThan, LessThan

### Sort
- field: String
- direction: SortDirection

### SortDirection
- Enum: Ascending, Descending

### Page
- page_number: u32
- items_per_page: u32

### Pagination
- total_items: u64
- total_pages: u32
- current_page: u32
- items_per_page: u32

## Error Handling
The module should define a custom Error type that encapsulates various error scenarios, such as:
- ScriptNotFound
- InvalidScriptContent
- ExecutionFailed
- IOError

## File Management
The module should handle reading and writing script files to the filesystem, ensuring proper file naming and directory structure. The script's path within the designated scripts folder serves as its unique identifier.

## Concurrency
The module should be designed to handle concurrent access to scripts, potentially using file system locks or other synchronization mechanisms to prevent conflicts.

