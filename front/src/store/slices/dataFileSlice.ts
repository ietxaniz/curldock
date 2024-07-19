import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { DataFileDetails } from "@/api/types";

interface DataFileState {
  files: DataFileDetails[];
}

const initialState: DataFileState = {
  files: [],
};

const dataFileSlice = createSlice({
  name: "dataFile",
  initialState,
  reducers: {
    addDataFile: (state, action: PayloadAction<DataFileDetails>) => {
      state.files.push(action.payload);
    },
    updateDataFileContent: (state, action: PayloadAction<{ fullName: string; content: Record<string, string>; createdAt: number; updatedAt: number }>) => {
      const file = state.files.find(f => f.fullName === action.payload.fullName);
      if (file) {
        file.content = action.payload.content;
        file.updatedAt = action.payload.updatedAt;
        if (file.createdAt !== action.payload.createdAt) {
          console.warn(`${file.fullName} created at changed from ${file.createdAt} to ${action.payload.createdAt}`);
        }
        file.createdAt = action.payload.createdAt;
      }
    },
    removeDataFile: (state, action: PayloadAction<string>) => {
      state.files = state.files.filter(f => f.fullName !== action.payload);
    },
    renameDataFile: (state, action: PayloadAction<{ fullName: string; newFullName: string }>) => {
      const file = state.files.find(f => f.fullName === action.payload.fullName);
      if (file) {
        file.fullName = action.payload.newFullName;
        file.updatedAt = Date.now();
      }
    },
    reorderDataFiles: (state, action: PayloadAction<null>) => {
      state.files.sort((a, b) => a.fullName.localeCompare(b.fullName));
    },
  },
});

export const { addDataFile, updateDataFileContent, removeDataFile, renameDataFile, reorderDataFiles } = dataFileSlice.actions;
const dataFileReducer = dataFileSlice.reducer;
export default dataFileReducer;

