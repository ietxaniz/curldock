import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { TreeItem } from "react-complex-tree";

export interface ItemData {
  name: string;
  editing: boolean;
  idx: number;
  path: string;
}

export interface FileexplorerState {
  treeData: TreeItem<ItemData>[];
  expandedItems: string[];
  loaded: boolean;
  editingItem: string;
  currentFileId: number;
}

const initialState: FileexplorerState = {
  treeData: [],
  expandedItems: [],
  loaded: false,
  editingItem: "",
  currentFileId: -1,
};

const fileexplorerSlice = createSlice({
  name: "fileexplorer",
  initialState,
  reducers: {
    setLoaded(state, action: PayloadAction<boolean>) {
      state.loaded = action.payload;
    },
    setTreeData(state, action: PayloadAction<TreeItem<ItemData>[]>) {
      state.treeData = action.payload;
    },
    expandItem(state, action: PayloadAction<string>) {
      state.expandedItems.push(action.payload);
    },
    collapseItem(state, action: PayloadAction<string>) {
      state.expandedItems = state.expandedItems.filter((id) => id !== action.payload);
    },
    renameItem(state, action: PayloadAction<{ item: TreeItem<ItemData>; newName: string }>) {
      const { item, newName } = action.payload;
      const index = state.treeData.findIndex((treeItem) => treeItem.index === item.index);

      if (index !== -1) {
        state.treeData = state.treeData.map((treeItem, idx) => {
          if (idx === index) {
            return {
              ...treeItem,
              data: {
                ...treeItem.data,
                name: newName,
              },
            };
          }
          return treeItem;
        });

        // If it's a folder, update paths of all children
        if (item.isFolder) {
          const oldPath = item.data.path + "/" + item.data.name;
          const newPath = item.data.path + "/" + newName;
          state.treeData = state.treeData.map((treeItem) => {
            if (treeItem.data.path.startsWith(oldPath)) {
              return {
                ...treeItem,
                data: {
                  ...treeItem.data,
                  path: treeItem.data.path.replace(oldPath, newPath),
                },
              };
            }
            return treeItem;
          });
        }
      }
    },
    setEditingItem(state, action: PayloadAction<string>) {
      state.editingItem = action.payload;
    },
    setCurrentFileId(state, action: PayloadAction<number>) {
      state.currentFileId = action.payload;
    },
    addFileToTree(state, action: PayloadAction<{ name: string; path: string; isFolder: boolean }>) {
      const { name, path, isFolder } = action.payload;
      const newIndex = state.treeData.length;
      const parentIndex = state.treeData.findIndex((item) => item.data.path === path);

      state.treeData.push({
        index: newIndex,
        children: [],
        isFolder,
        data: { name, editing: false, idx: newIndex, path },
      });

      if (parentIndex !== -1 && state.treeData[parentIndex].children) {
        state.treeData[parentIndex].children.push(newIndex);
      }
    },
  },
});

export const { setLoaded, setTreeData, expandItem, collapseItem, renameItem, setEditingItem, setCurrentFileId, addFileToTree } = fileexplorerSlice.actions;

const fileexplorerReducer = fileexplorerSlice.reducer;
export default fileexplorerReducer;
export type { TreeItem };
