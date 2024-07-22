import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { TreeItem, TreeItemIndex } from "react-complex-tree";

import { FileType } from "@/api/types";

export interface ItemData {
  name: string;
  editing: boolean;
  path: string;
  itemType: FileType;
}

export interface TreeMap {
  [key: string]: TreeItem<ItemData>;
}

export interface FileexplorerState {
  treeData: TreeMap;
  expandedItems: string[];
  loaded: boolean;
  editingItem: string;
  currentFileId: number;
}

const initialState: FileexplorerState = {
  treeData: {},
  expandedItems: [],
  loaded: false,
  editingItem: "",
  currentFileId: -1,
};

const getItemKey = (item: ItemData): string => {
  if (item.path.length > 0) {
    return item.path + "/" + item.name;
  }
  return item.name;
};

const getParentPath = (path: string, name: string):string => {
  const newPath = path.substring(0, path.length - name.length);
  if (newPath.length === 0) {
    return newPath;
  }
  if (newPath[newPath.length-1] === "/") {
    return newPath.substring(0, newPath.length-1);
  }
  return newPath;
}

const computeFullName = (path: string, name: string): string => {
  if (path.length > 0) {
    return path + "/" + name;
  }
  return name;
};

const sortChildrenOfItem = (map: TreeMap, itemKey: string, recursive: boolean) => {
  const item = map[itemKey];
  if (!item) {
    return;
  }
  if (!item.isFolder) {
    return;
  }
  if (!item.children) {
    item.children = [];
    return;
  }
  item.children.sort((a: TreeItemIndex, b: TreeItemIndex) => {
    const itemA = map[a.toString()];
    const itemB = map[b.toString()];

    if (itemA.isFolder !== itemB.isFolder) {
      return itemA.isFolder ? -1 : 1;
    }
    return itemA.data.name.localeCompare(itemB.data.name);
  });
  if (recursive) {
    for (let i = 0; i < item.children.length; i++) {
      sortChildrenOfItem(map, item.children[i].toString(), recursive);
    }
  }
};

const renamePathRecursive = (map: TreeMap, oldPath: string, newPath: string, name: string) => {
  const fullOldPath = computeFullName(oldPath, name);
  const fullNewPath = computeFullName(newPath, name);
  const item = map[fullOldPath];
  delete map[fullOldPath];
  map[fullNewPath] = item;
  item.index = fullNewPath;
  item.data.path = newPath;
  if (item.isFolder) {
    for (let i = 0; i < item.children!.length; i++) {
      const childItem = map[item.children![i]];
      renamePathRecursive(map, fullOldPath, fullNewPath, childItem.data.name);
    }
  }
};

const deleteRecursive = (map: TreeMap, key: string) => {
  const item = map[key];
  if (!item) {
    return;
  }
  delete map[key];
  if (!item.children) {
    return;
  }
  for (let i = 0; i < item.children.length; i++) {
    deleteRecursive(map, item.children[i].toString());
  }
};

const fileexplorerSlice = createSlice({
  name: "fileexplorer",
  initialState,
  reducers: {
    setLoaded(state, action: PayloadAction<boolean>) {
      state.loaded = action.payload;
    },
    setTreeData(state, action: PayloadAction<ItemData[]>) {
      const items = action.payload;
      const treeData:TreeMap = {};
      treeData[''] = {
        index: '', children: [], isFolder: true, canMove: false, canRename: false, data: {name: '', editing: false, path: '', itemType: FileType.Folder}
      }
      for (let i = 0; i < items.length; i++) {
        const item = items[i];
        const key = item.path;
        const path = getParentPath(item.path, item.name);
        treeData[key] = {
          index: key,
          isFolder: item.itemType === FileType.Folder,
          children: item.itemType === FileType.Folder ? [] : undefined,
          canMove: key !== "",
          canRename: key !== "",
          data: item,
        };
        treeData[path].children!.push(key);
      }
      sortChildrenOfItem(treeData, "", true);
      state.treeData = treeData;
    },
    expandItem(state, action: PayloadAction<string>) {
      state.expandedItems.push(action.payload);
    },
    collapseItem(state, action: PayloadAction<string>) {
      state.expandedItems = state.expandedItems.filter((id) => id !== action.payload);
    },
    renameItem(state, action: PayloadAction<{ path: string; oldName: string; newName: string }>) {
      const { path, oldName, newName } = action.payload;
      const fullOldPath = computeFullName(path, oldName);
      const fullNewPath = computeFullName(path, newName);
      const item = state.treeData[fullOldPath];
      delete state.treeData[fullOldPath];
      state.treeData[fullNewPath] = item;
      item.data.name = newName;
      item.index = fullNewPath;
      if (item.isFolder) {
        for (let i = 0; i < item.children!.length; i++) {
          const childItem = state.treeData[item.children![i]];
          renamePathRecursive(state.treeData, oldName, newName, childItem.data.name);
        }
      }
    },
    setEditingItem(state, action: PayloadAction<string>) {
      state.editingItem = action.payload;
    },
    setCurrentFileId(state, action: PayloadAction<number>) {
      state.currentFileId = action.payload;
    },
    addItemToTree(state, action: PayloadAction<{ name: string; path: string; itemType: FileType }>) {
      const { name, path, itemType } = action.payload;
      const parent = state.treeData[path];
      if (!parent) {
        return;
      }
      if (!parent.isFolder || !parent.children) {
        return;
      }
      const key = computeFullName(path, name);
      if (state.treeData[key]) {
        return;
      }
      state.treeData[key] = {
        index: key,
        isFolder: itemType === FileType.Folder,
        children: itemType === FileType.Folder ? [] : undefined,
        canMove: key !== "",
        canRename: key !== "",
        data: {
          name: name,
          path: path,
          editing: false,
          itemType: itemType,
        },
      };
      parent.children.push(key);
      sortChildrenOfItem(state.treeData, path, false);
    },
    deleteItemFromTree(state, action: PayloadAction<{ key: string }>) {
      deleteRecursive(state.treeData, action.payload.key);
    },
  },
});


export const { setLoaded, setTreeData, expandItem, collapseItem, renameItem, setEditingItem, setCurrentFileId, addItemToTree, deleteItemFromTree } = fileexplorerSlice.actions;

const fileexplorerReducer = fileexplorerSlice.reducer;
export default fileexplorerReducer;
export type { TreeItem };