import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { ScriptDetails } from "@/api/types";

// Interface for script details
export interface ScriptInputState {
  [key: string]: ScriptDetails;
}

const initialState: ScriptInputState = {};

const scriptInputSlice = createSlice({
  name: 'scriptInput',
  initialState,
  reducers: {
    setScript: (state, action: PayloadAction<ScriptDetails>) => {
      state[action.payload.fullName] = action.payload;
    },
    renameScript: (state, action: PayloadAction<{ oldName: string; newName: string }>) => {
      const { oldName, newName } = action.payload;
      const item = state[oldName];
      if (item) {
        state[newName] = { ...item, fullName: newName };
        delete state[oldName];
      }
    },
    removeScript: (state, action: PayloadAction<string>) => {
      delete state[action.payload];
    },
  },
});

export const { setScript, renameScript, removeScript } = scriptInputSlice.actions;
const scriptInputReducer = scriptInputSlice.reducer;
export default scriptInputReducer;

