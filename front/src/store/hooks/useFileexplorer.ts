import { useAppDispatch, useAppSelector } from "./hooks";
import {
  setLoaded,
  setTreeData,
  expandItem,
  collapseItem,
  renameItem,
  setEditingItem,
  TreeItem,
  ItemData,
  setCurrentFileId,
  addFileToTree,
} from "../slices/fileexplorerSlice";

export const useGetEditingItem = (): string => {
  return useAppSelector((state) => state.fileexplorer.editingItem);
};

export const useSetEditingItem = (): ((itemId: string) => void) => {
  const dispatch = useAppDispatch();
  return (itemId: string) => dispatch(setEditingItem(itemId));
};

export const useGetLoaded = () => {
  return useAppSelector((state) => state.fileexplorer.loaded);
};

export const useSetLoaded = (): ((loaded: boolean) => void) => {
  const dispatch = useAppDispatch();
  const setLoadedDispatch = (loaded: boolean) => {
    dispatch(setLoaded(loaded));
  };
  return setLoadedDispatch;
};

export const useGetTreeData = () => {
  return useAppSelector((state) => state.fileexplorer.treeData);
};

export const useSetTreeData = (): ((treeData: TreeItem<ItemData>[]) => void) => {
  const dispatch = useAppDispatch();
  const setTreeDataDispatch = (treeData: TreeItem<ItemData>[]) => {
    dispatch(setTreeData(treeData));
  };
  return setTreeDataDispatch;
};

export const useGetExpandedItems = () => {
  return useAppSelector((state) => state.fileexplorer.expandedItems);
};

export const useExpandItem = (): ((itemId: string) => void) => {
  const dispatch = useAppDispatch();
  const expandItemDispatch = (itemId: string) => {
    dispatch(expandItem(itemId));
  };
  return expandItemDispatch;
};

export const useCollapseItem = (): ((itemId: string) => void) => {
  const dispatch = useAppDispatch();
  const collapseItemDispatch = (itemId: string) => {
    dispatch(collapseItem(itemId));
  };
  return collapseItemDispatch;
};

export const useRenameItem = (): ((item: TreeItem<ItemData>, newName: string) => void) => {
  const dispatch = useAppDispatch();
  const renameItemDispatch = (item: TreeItem<ItemData>, newName: string) => {
    dispatch(renameItem({ item, newName }));
  };
  return renameItemDispatch;
};

export const useGetCurrentFileId = () => useAppSelector((state) => state.fileexplorer.currentFileId);

export const useSetCurrentFileId = () => {
  const dispatch = useAppDispatch();
  return (id: number) => dispatch(setCurrentFileId(id));
};

export const useAddFileToTree = (): ((name: string, path: string, isFolder: boolean) => void) => {
  const dispatch = useAppDispatch();
  return (name: string, path: string, isFolder: boolean) => {
    dispatch(addFileToTree({ name, path, isFolder }));
  };
};
