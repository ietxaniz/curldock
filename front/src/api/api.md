## Code on src/api folder of curldock frontend project


~~~apiClient.ts
import { Response } from "./types";

const BASE_URL = '/api/v1';

const apiClient = {
  get: async <T>(url: string, params?: Record<string, any>): Promise<Response<T>> => {
    const query = params ? `?${new URLSearchParams(params)}` : '';
    const response = await fetch(`${BASE_URL}${url}${query}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
    });
    return response.json();
  },

  post: async <T>(url: string, body: any): Promise<Response<T>> => {
    const response = await fetch(`${BASE_URL}${url}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    return response.json();
  },

  put: async <T>(url: string, body: any): Promise<Response<T>> => {
    const response = await fetch(`${BASE_URL}${url}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    return response.json();
  },

  delete: async <T>(url: string, params?: Record<string, any>): Promise<Response<T>> => {
    const query = params ? `?${new URLSearchParams(params)}` : '';
    const response = await fetch(`${BASE_URL}${url}${query}`, {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
    });
    return response.json();
  }
};

export default apiClient;

~~~

~~~base.ts
export interface Response<T> {
  data?: T;
  error?: {
    errorType: string;
    errorDetails: string;
  };
}

export const checkError = <T>(response: Response<T>): T => {
  if (response.error) {
    throw new Error(`${response.error.errorType}: ${response.error.errorDetails}`);
  }
  if (!response.data) {
    throw new Error("No data available");
  }
  return response.data;
};

export enum HttpMethod {
  GET = "GET",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  PATCH = "PATCH",
  HEAD = "HEAD",
  OPTIONS = "OPTIONS"
}

export interface CurlOptions {
  verbose?: boolean;
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
  timeNamelookup?: boolean;
  timeConnect?: boolean;
  timeAppconnect?: boolean;
  timePretransfer?: boolean;
  timeStarttransfer?: boolean;
  timeTotal?: boolean;
}

export interface CurlCommand {
  method: HttpMethod;
  url: string;
  headers: Array<[string, string]>;
  data?: string;
  cookies?: Array<[string, string]>;
  options: CurlOptions;
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
}

export interface ScriptDetails {
  name: string;
  path: string;
  curlCommand: CurlCommand;
  createdAt: number;
  updatedAt: number;
}

export interface ScriptDetailsCreate {
  name: string;
  path: string;
  curlCommand: CurlCommand;
}

export interface ScriptInfo {
  name: string;
  createdAt: number;
  updatedAt: number;
  isFolder: boolean;
  path: string;
}

export interface ScriptList {
  path: string;
  scripts: ScriptInfo[];
}

export interface FolderCreate {
  path: string;
}

~~~

~~~endpoints/asset.ts
import apiClient from '../apiClient';
import { Response, checkError } from '../types';

/**
 * Delete a file or folder
 * @param path - The path of the file or folder to delete
 * @returns void
 */
export const deleteAsset = async (path: string): Promise<void> => {
  const response = await apiClient.delete<void>('/asset', { path });
  return checkError(response);
};

~~~

~~~endpoints/data.ts
// src/api/endpoints/data.ts

import apiClient from '../apiClient';
import { Response, checkError, DataFileDetails } from '../types';

/**
 * Load a data file
 * @param path - The path of the data file to load
 * @returns The details of the loaded data file
 */
export const loadDataFile = async (path: string): Promise<DataFileDetails> => {
  const response = await apiClient.get<DataFileDetails>('/data', { path });
  return checkError(response);
};

/**
 * Store a new data file
 * @param path - The path where the data file will be stored
 * @param content - The content of the data file
 * @returns The details of the stored data file
 */
export const storeDataFile = async (path: string, content: Record<string, string>): Promise<DataFileDetails> => {
  const response = await apiClient.post<DataFileDetails>('/data', { path, content });
  return checkError(response);
};

/**
 * Update an existing data file
 * @param path - The path of the data file to update
 * @param content - The new content of the data file
 * @returns The details of the updated data file
 */
export const updateDataFile = async (path: string, content: Record<string, string>): Promise<DataFileDetails> => {
  const response = await apiClient.put<DataFileDetails>('/data', { path, content });
  return checkError(response);
};

/**
 * Delete a data file
 * @param path - The path of the data file to delete
 * @returns void
 */
export const deleteDataFile = async (path: string): Promise<void> => {
  const response = await apiClient.delete<void>('/asset', { path });
  return checkError(response);
};

/**
 * Rename a data file
 * @param oldPath - The current path of the data file
 * @param newPath - The new path for the data file
 * @returns void
 */
export const renameDataFile = async (oldPath: string, newPath: string): Promise<void> => {
  const response = await apiClient.post<void>('/data/rename', { oldPath, newPath });
  return checkError(response);
};

~~~

~~~endpoints/execute.ts
import apiClient from '../apiClient';
import { checkError, CurlCommand } from '../types';

/**
 * Execute a script
 * @param path - The path of the script to execute
 * @returns The result of the executed script
 */
export const executeScript = async (path: string): Promise<CurlCommand> => {
  const response = await apiClient.post<CurlCommand>('/execute', { path });
  return checkError(response);
};

~~~

~~~endpoints/folder.ts
import apiClient from '../apiClient';
import { Response, checkError } from '../types';

/**
 * Create a new folder
 * @param path - The path of the folder to create
 * @returns void
 */
export const createFolder = async (path: string): Promise<void> => {
  const response = await apiClient.post<void>('/folder', { path });
  return checkError(response);
};

~~~

~~~endpoints/list.ts
import apiClient from '../apiClient';
import { Response, checkError, FileList } from '../types';

/**
 * List all files and folders recursively
 * @returns The list of all files and folders
 */
export const listAllFiles = async (): Promise<FileList> => {
  const response = await apiClient.get<FileList>('/list');
  return checkError(response);
};

~~~

~~~endpoints/script.ts
import apiClient from '../apiClient';
import { Response, checkError, ScriptDetails, ScriptDetailsCreate } from '../types';

/**
 * Get details of a script
 * @param path - The path of the script
 * @returns The details of the script
 */
export const getScriptDetails = async (path: string): Promise<ScriptDetails> => {
  const response = await apiClient.get<ScriptDetails>('/script', { path });
  return checkError(response);
};

/**
 * Create a new script
 * @param script - The details of the script to create
 * @returns The details of the created script
 */
export const createScript = async (script: ScriptDetailsCreate): Promise<ScriptDetails> => {
  const response = await apiClient.post<ScriptDetails>('/script', script);
  return checkError(response);
};

/**
 * Update an existing script
 * @param script - The details of the script to update
 * @returns The details of the updated script
 */
export const updateScript = async (script: ScriptDetailsCreate): Promise<ScriptDetails> => {
  const response = await apiClient.put<ScriptDetails>('/script', script);
  return checkError(response);
};

/**
 * Rename a script
 * @param oldFullName - The current full name of the script
 * @param newFullName - The new full name for the script
 * @returns The details of the renamed script
 */
export const renameScript = async (oldFullName: string, newFullName: string): Promise<ScriptDetails> => {
  const response = await apiClient.post<ScriptDetails>('/script/rename', { oldFullName, newFullName });
  return checkError(response);
};

~~~

~~~types.ts
// src/api/types.ts

// Enum for HTTP methods
export enum HttpMethod {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS
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
  Script,
  Data
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

~~~
