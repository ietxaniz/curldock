# CurlDock Curl Manager Module

The Curl Manager module is responsible for handling all curl-related operations in CurlDock.

## Functions

### list_scripts
- Input: path: Option<String>
- Output: Result<ScriptList, Error>
- Description: Lists all scripts in the specified path (or root if None).

### create_script
- Input: path: Option<String>, name: String, content: String
- Output: Result<Script, Error>
- Description: Creates a new curl script with the given name and content in the specified path.

### update_script
- Input: path: String, name: String, content: String
- Output: Result<Script, Error>
- Description: Updates the script with the given name in the specified path.

### delete_script
- Input: path: String, name: String
- Output: Result<bool, Error>
- Description: Deletes the script with the given name from the specified path.

### get_script_details
- Input: path: String, name: String
- Output: Result<ScriptDetails, Error>
- Description: Retrieves all information about the script with the given name from the specified path.

### execute_script
- Input: path: String, name: String
- Output: Result<ExecutionResult, Error>
- Description: Executes the script with the given name from the specified path and returns the execution result.

## Data Structures

### ScriptList
- path: String
- scripts: Vec<ScriptInfo>

### ScriptInfo
- name: String
- created_at: DateTime<Utc>
- updated_at: DateTime<Utc>
- is_folder: bool

### Script
- name: String
- content: String
- created_at: DateTime<Utc>

### ScriptDetails
- name: String
- path: String
- content: String
- created_at: DateTime<Utc>
- updated_at: DateTime<Utc>

### ExecutionResult
- name: String
- output: String
- executed_at: DateTime<Utc>

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