# API Documentation for /api/v1

## Chapter 1: API Involved Structures (TypeScript)

```typescript
interface CurlCommand {
  method: HttpMethod;
  url: string;
  headers: [string, string][];
  data?: string;
  cookies: [string, string][];
  options: CurlOptions;
  storeCurlBody: StoreCurlBody[];
  storeCurlCookie: StoreCurlCookie[];
  loadCurl: LoadCurl[];
}

enum HttpMethod {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS
}

interface CurlOptions {
  insecure?: boolean;
  followRedirects?: boolean;
  maxRedirects?: number;
  timeout?: number;
  connectTimeout?: number;
  proxy?: string;
  outputFile?: string;
  cert?: string;
  key?: string;
  keyPassword?: string;
  compressed?: boolean;
  retry?: number;
  retryDelay?: number;
  fail?: boolean;
  interface?: string;
  dnsServers?: string[];
  ipv4Only?: boolean;
  ipv6Only?: boolean;
  maxTime?: number;
  rateLimit?: number;
}

interface StoreCurlBody {
  source: string;
  destination: string;
  filename: string;
}

interface StoreCurlCookie {
  source: string;
  destination: string;
  filename: string;
}

interface LoadCurl {
  filename: string;
  dataName: string;
  envVariable: string;
}

interface ScriptDetails {
  fullName: string;
  curlCommand: CurlCommand;
  createdAt: number;
  updatedAt: number;
}

interface ScriptDetailsCreate {
  fullName: string;
  curlCommand: CurlCommand;
}

interface FileInfo {
  name: string;
  createdAt: number;
  updatedAt: number;
  isFolder: boolean;
  path: string;
  fileType: FileType;
}

enum FileType {
  Script,
  Data,
  Folder,
  Unknown
}

interface FileList {
  path: string;
  files: FileInfo[];
}

interface DataFileDetails {
  fullName: string;
  content: Record<string, string>;
  createdAt: number;
  updatedAt: number;
}

interface Response<T> {
  data?: T;
  error?: ErrorResponse;
}

interface ErrorResponse {
  errorType: string;
  errorDetails: string;
}
```

## Chapter 2: List of API Endpoints

1. **GET /api/v1/data**
   - Description: Load a data file
   - Input: Query parameter `path` (string)
   - Output: `Response<DataFileDetails>`

2. **POST /api/v1/data**
   - Description: Store a new data file
   - Input: Body of type `{ path: string, content: Record<string, string> }`
   - Output: `Response<DataFileDetails>`

3. **PUT /api/v1/data**
   - Description: Update an existing data file
   - Input: Body of type `{ path: string, content: Record<string, string> }`
   - Output: `Response<DataFileDetails>`

4. **DELETE /api/v1/asset**
   - Description: Delete a data file, script file or folder
   - Input: Query parameter `path` (string)
   - Output: `Response<void>`

5. **POST /api/v1/data/rename**
   - Description: Rename a file (data file or script)
   - Input: Body of type `{ oldPath: string, newPath: string }`
   - Output: `Response<void>`

6. **POST /api/v1/execute**
   - Description: Execute a script
   - Input: Query parameter `path` (string)
   - Output: `Response<CurlCommandResult>`

7. **POST /api/v1/folder**
   - Description: Create a new folder
   - Input: Body of type `{ path: string }`
   - Output: `Response<void>`

8. **GET /api/v1/list**
   - Description: List all files and folders recursively
   - Input: None
   - Output: `Response<FileList>`

9. **GET /api/v1/script**
   - Description: Get details of a script
   - Input: Query parameter `path` (string)
   - Output: `Response<ScriptDetails>`

10. **POST /api/v1/script**
    - Description: Create a new script
    - Input: Body of type `ScriptDetailsCreate`
    - Output: `Response<ScriptDetails>`

11. **PUT /api/v1/script**
    - Description: Update an existing script
    - Input: Body of type `ScriptDetailsCreate`
    - Output: `Response<ScriptDetails>`

12. **POST /api/v1/script/rename**
    - Description: Rename a script
    - Input: Body of type `{ oldFullName: string, newFullName: string }`
    - Output: `Response<ScriptDetails>`

Note: All endpoints return a `Response` object which may contain either the expected data or an error. The specific type of data returned is indicated for each endpoint.
