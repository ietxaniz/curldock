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
