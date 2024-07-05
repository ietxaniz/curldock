import { configureStore } from "@reduxjs/toolkit";
import fileexplorerReducer from './slices/fileexplorerSlice'


export const store = configureStore({
  reducer: {
    fileexplorer: fileexplorerReducer,
  },
  devTools: false,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
