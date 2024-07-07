import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface CurlCommand {
  method: HttpMethod;
  url: string;
  headers: Array<[string, string]>;
  data?: string;
  options: CurlOptions;
}

export interface CurlOptions {
  verbose: boolean;
  insecure: boolean;
}

export enum HttpMethod {
  GET = "GET",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  PATCH = "PATCH",
  HEAD = "HEAD",
  OPTIONS = "OPTIONS"
}

export interface CurlCommandResult {
  request: CurlCommand;
  responseHeaders: { [key: string]: string };
  statusCode: number;
  date: string;
  body: string;
}

export interface ScriptDetails {
  name: string;
  path: string;
  curlCommand: CurlCommand;
  createdAt: string;
  updatedAt: string;
}

export interface ScriptDetailsCreate {
  name: string;
  path: string;
  curlCommand: CurlCommand;
}

export interface Curl {
  fileId: number;
  script: CurlCommand;
  result?: CurlCommandResult;
}

export interface CurlState {
  fileIds: number[];
  curlItems: Curl[];
}

const initialState: CurlState = {
  fileIds: [],
  curlItems: [],
};

export const curlSlice = createSlice({
  name: "curl",
  initialState,
  reducers: {
    addCurlItem: (state, action: PayloadAction<{curl:Curl, fileId: number}>) => {
      if (state.fileIds.indexOf(action.payload.fileId) >= 0) {
        return;
      }
      state.curlItems.push(action.payload.curl);
      state.fileIds.push(action.payload.fileId);
    },
    updateCurlItem: (state, action: PayloadAction<Curl>) => {
      const index = state.curlItems.findIndex(item => item.fileId === action.payload.fileId);
      if (index !== -1) {
        state.curlItems[index] = action.payload;
      }
    },
    removeCurlItem: (state, action: PayloadAction<number>) => {
      state.curlItems = state.curlItems.filter(item => item.fileId !== action.payload);
      state.fileIds = state.fileIds.filter(id => id !== action.payload);
    },
  },
});

export const { addCurlItem, updateCurlItem, removeCurlItem } = curlSlice.actions;
const curlReducer = curlSlice.reducer;
export default curlReducer;

