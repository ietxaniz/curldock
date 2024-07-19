// src/api/types.ts

// Enum for HTTP methods
export enum HttpMethod {
  GET="GET",
  POST="POST",
  PUT="PUT",
  DELETE="DELETE",
  PATCH="PATCH",
  HEAD="HEAD",
  OPTIONS="OPTIONS"
}

// Interface for Curl options
export interface CurlOptions {
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

// Interface for storing Curl body
export interface StoreCurlBody {
  source: string;
  destination: string;
  filename: string;
}

// Interface for storing Curl cookies
export interface StoreCurlCookie {
  source: string;
  destination: string;
  filename: string;
}

// Interface for loading Curl
export interface LoadCurl {
  filename: string;
  dataName: string;
  envVariable: string;
}

// Interface for Curl command
export interface CurlCommand {
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

// Interface for script details
export interface ScriptDetails {
  fullName: string;
  curlCommand: CurlCommand;
  createdAt: number;
  updatedAt: number;
}

// Interface for creating script details
export interface ScriptDetailsCreate {
  fullName: string;
  curlCommand: CurlCommand;
}

// Enum for file types
export enum FileType {
  Script = "Script",
  Data = "Data", 
  Folder = "Folder",
  Unknown = "Unknown"
}

// Interface for file information
export interface FileInfo {
  name: string;
  createdAt: number;
  updatedAt: number;
  isFolder: boolean;
  path: string;
  fileType: FileType;
}

// Interface for file list
export interface FileList {
  path: string;
  files: FileInfo[];
}

// Interface for data file details
export interface DataFileDetails {
  fullName: string;
  content: Record<string, string>;
  createdAt: number;
  updatedAt: number;
}

// Generic response interface
export interface Response<T> {
  data?: T;
  error?: ErrorResponse;
}

// Interface for error response
export interface ErrorResponse {
  errorType: string;
  errorDetails: string;
}

export interface StoreData {
  parameter: string;
  filename: string;
  data: string;
}

export interface CurlCommandResult {
  request: CurlCommand;
  responseHeaders: { [key: string]: string }; 
  statusCode: number; 
  date: string;
  body: string;
  cookies: { [key: string]: string }; 
  contentType?: string; 
  redirectCount?: number; 
  effectiveUrl?: string;
  timeNamelookup?: number;
  timeConnect?: number;
  timeAppconnect?: number;
  timePretransfer?: number;
  timeStarttransfer?: number;
  timeTotal?: number;
  storeData: StoreData[];
}

// Utility function to check for errors in the response
export const checkError = <T>(response: Response<T>): T => {
  if (response.error) {
    throw new Error(`${response.error.errorType}: ${response.error.errorDetails}`);
  }
  if (!response.data) {
    throw new Error("No data available");
  }
  return response.data;
};

export const getPathNameFromFullName = (fullName: string): {path:string, name: string} => {
  const lastSlashIndex = fullName.lastIndexOf('/');
  
  if (lastSlashIndex === -1) {
    return { path: '', name: fullName }; // No path, just the name
  }

  const path = fullName.substring(0, lastSlashIndex);
  const name = fullName.substring(lastSlashIndex + 1);

  return { path, name };
};