import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  headerHeight: 0
};

const layoutSlice = createSlice({
  name: 'layout',
  initialState,
  reducers: {
    setHeaderHeight: (state, action) => {
      state.headerHeight = action.payload;
    }
  }
});

export const { setHeaderHeight } = layoutSlice.actions;
const layoutReducer = layoutSlice.reducer;
export default layoutReducer;
