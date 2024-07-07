import { configureStore } from "@reduxjs/toolkit";
import fileexplorerReducer from './slices/fileexplorerSlice'
import curlReducer from "./slices/curlSlice";
import layoutReducer from "./slices/layoutSlice";


export const store = configureStore({
  reducer: {
    fileexplorer: fileexplorerReducer,
    curl: curlReducer,
    layout: layoutReducer,
  },
  devTools: false,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
