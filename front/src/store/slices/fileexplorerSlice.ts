import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { TreeItem, TreeItemIndex } from "react-complex-tree";

import { FileType } from "@/api/types";

export interface ItemData {
  name: string;
  editing: boolean;
  idx: number;
  path: string;
  itemType: FileType;
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

const getItemByIndex = (items:TreeItem<ItemData>[], idx: number) => {
  const index = items.findIndex((treeItem) => { treeItem.data.idx === idx});
  if (index >= 0) {
    return items[index];
  }
}

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
        if (item.data.itemType === FileType.Folder) {
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
    addFileToTree(state, action: PayloadAction<{ name: string; path: string; itemType: FileType }>) {
      const { name, path, itemType } = action.payload;
      const newIndex = state.treeData.length;
      let parentIndex: number;
      if (path === "") {
        parentIndex = 0;
      } else {
        parentIndex = state.treeData.findIndex((item) => {
          if (item.data.itemType !== FileType.Folder) return false;

          const itemFullPath = item.data.path ? `${item.data.path}/${item.data.name}` : item.data.name;

          return itemFullPath === path;
        });
      }

      state.treeData.push({
        index: newIndex,
        children: [],
        isFolder: itemType === FileType.Folder,
        data: { name, editing: false, idx: newIndex, path, itemType },
      });

      if (parentIndex !== -1 && state.treeData[parentIndex].children) {
        state.treeData[parentIndex].children!.push(newIndex);

        state.treeData[parentIndex].children!.sort((a:TreeItemIndex, b:TreeItemIndex) => {
          const itemA = state.treeData[Number(a)];
          const itemB = state.treeData[Number(b)];
          
          // Folders come before files
          if (itemA.isFolder !== itemB.isFolder) {
            return itemA.isFolder ? -1 : 1;
          }
          
          // Alphabetical order for items of the same type
          return itemA.data.name.localeCompare(itemB.data.name);
        });
      }
    },
  },
});

export const { setLoaded, setTreeData, expandItem, collapseItem, renameItem, setEditingItem, setCurrentFileId, addFileToTree } = fileexplorerSlice.actions;

const fileexplorerReducer = fileexplorerSlice.reducer;
export default fileexplorerReducer;
export type { TreeItem };
