import { configureStore } from "@reduxjs/toolkit";
import fileexplorerReducer from './slices/fileexplorerSlice'
import curlReducer from "./slices/curlSlice";
import layoutReducer from "./slices/layoutSlice";
import dataFileReducer from "./slices/dataFileSlice";
import scriptInputReducer from "./slices/scriptInputSlice";
import scriptResultSlice from "./slices/scriptResultSlice";


export const store = configureStore({
  reducer: {
    fileexplorer: fileexplorerReducer,
    curl: curlReducer,
    layout: layoutReducer,
    dataFile: dataFileReducer,
    scriptInput: scriptInputReducer,
    scriptResult: scriptResultSlice,
  },
  devTools: false,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
