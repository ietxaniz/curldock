import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { CurlCommand, CurlCommandResult, ScriptDetails, ScriptDetailsCreate, HttpMethod, CurlOptions } from "../../api/base";

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
    addCurlItem: (state, action: PayloadAction<{curl: Curl, fileId: number}>) => {
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
    updateCurlResult: (state, action: PayloadAction<{fileId: number, result: CurlCommandResult}>) => {
      const index = state.curlItems.findIndex(item => item.fileId === action.payload.fileId);
      if (index !== -1) {
        state.curlItems[index].result = action.payload.result;
      }
    },
  },
});

export const { addCurlItem, updateCurlItem, removeCurlItem, updateCurlResult } = curlSlice.actions;
const curlReducer = curlSlice.reducer;
export default curlReducer;

// Re-export types from base.ts for convenience
export type { CurlCommand, CurlCommandResult, ScriptDetails, ScriptDetailsCreate, CurlOptions };
export { HttpMethod };
