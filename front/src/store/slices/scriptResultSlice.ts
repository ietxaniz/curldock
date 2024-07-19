import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { CurlCommandResult } from '../../api/types';

interface ScriptResultState {
  [key: string]: CurlCommandResult[];
}

const initialState = {} as ScriptResultState;

const scriptResultSlice = createSlice({
  name: 'scriptResult',
  initialState,
  reducers: {
    addResult: (state, action: PayloadAction<{ name: string; result: CurlCommandResult }>) => {
      const { name, result } = action.payload;
      if (state[name]) {
        state[name].push(result);
      } else {
        state[name] = [result];
      }
    },
    renameResult: (state, action: PayloadAction<{ oldName: string; newName: string }>) => {
      const { oldName, newName } = action.payload;
      const item = state[oldName];
      if (item) {
        state[newName] = item;
        delete state[oldName];
      }
    },
    removeAllResultsOfScript: (state, action: PayloadAction<string>) => {
      delete state[action.payload];
    },
    removeSpecificResult: (state, action: PayloadAction<{ name: string; index: number }>) => {
      const { name, index } = action.payload;
      if (state[name] && state[name][index]) {
        state[name].splice(index, 1);
      }
    },
  },
});

export const { addResult, renameResult, removeAllResultsOfScript, removeSpecificResult } = scriptResultSlice.actions;
export default scriptResultSlice.reducer;
