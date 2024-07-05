import { configureStore } from "@reduxjs/toolkit";
import fileexplorerReducer from './slices/fileexplorerSlice'
import curlReducer from "./slices/curlSlice";


export const store = configureStore({
  reducer: {
    fileexplorer: fileexplorerReducer,
    curl: curlReducer,
  },
  devTools: false,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
