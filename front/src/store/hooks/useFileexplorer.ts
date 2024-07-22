import { useAppDispatch, useAppSelector } from "@/store/hooks/hooks";
import {
  setLoaded,
  setTreeData,
  expandItem,
  collapseItem,
  renameItem,
  setEditingItem,
  setCurrentFileId,
  addItemToTree,
  deleteItemFromTree,
  ItemData,
} from "@/store/slices/fileexplorerSlice";
import { FileType } from "@/api/types";
import { TreeItem } from "react-complex-tree";

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

export const useSetTreeData = (): ((treeData: ItemData[]) => void) => {
  const dispatch = useAppDispatch();
  const setTreeDataDispatch = (treeData: ItemData[]) => {
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

export const useRenameItem = (): ((path: string, oldName: string, newName: string) => void) => {
  const dispatch = useAppDispatch();
  const renameItemDispatch = (path: string, oldName: string, newName: string) => {
    dispatch(renameItem({ path, oldName, newName }));
  };
  return renameItemDispatch;
};

export const useGetCurrentFileId = () => useAppSelector((state) => state.fileexplorer.currentFileId);

export const useSetCurrentFileId = () => {
  const dispatch = useAppDispatch();
  return (id: number) => dispatch(setCurrentFileId(id));
};

export const useAddItemToTree = (): ((name: string, path: string, itemType: FileType) => void) => {
  const dispatch = useAppDispatch();
  return (name: string, path: string, itemType: FileType) => {
    dispatch(addItemToTree({ name, path, itemType }));
  };
};

export const useDeleteItemFromTree = (): ((key: string) => void) => {
  const dispatch = useAppDispatch();
  return (key: string) => {
    dispatch(deleteItemFromTree({ key }));
  };
};
