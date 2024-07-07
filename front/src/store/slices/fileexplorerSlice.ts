import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { TreeItem } from "react-complex-tree";

export interface ItemData {
  name: string;
  editing: boolean;
  idx: number;
  path: string;
}

export interface FileexplorerState {
  treeData: TreeItem<ItemData>[],
  expandedItems: string[];
  loaded: boolean;
  editingItem: string;
  currentFileId: number;
}

const initialState:FileexplorerState = {
  treeData: [],
  expandedItems: [],
  loaded: false,
  editingItem: '',
  currentFileId: -1,
};

const filexplorerSlice = createSlice({
  name: "fileexplorer",
  initialState,
  reducers: {
    setLoaded(state, action:PayloadAction<boolean>) {
      state.loaded = action.payload;
    },
    setTreeData(state, action: PayloadAction<TreeItem<ItemData>[]>) {
      state.treeData = action.payload;
    },
    expandItem(state, action:PayloadAction<string>) {
      state.expandedItems.push(action.payload);
    },
    collapseItem(state, action:PayloadAction<string> ) {
      state.expandedItems = state.expandedItems.filter(id => id !== action.payload);
    },
    renameItem(state, action:PayloadAction<{item: TreeItem<ItemData>, newName: string}>) {
      state.treeData[action.payload.item.data.idx].data.name = action.payload.newName;
    },
    setEditingItem(state, action: PayloadAction<string>) {
      state.editingItem = action.payload;
    },
    setCurrentFileId(state, action: PayloadAction<number>) {
      state.currentFileId = action.payload;
    },
  }
});

export const { setLoaded, setTreeData, expandItem, collapseItem, renameItem, setEditingItem, setCurrentFileId } = filexplorerSlice.actions;
const fileexplorerReducer = filexplorerSlice.reducer;
export default fileexplorerReducer;
export type { TreeItem };