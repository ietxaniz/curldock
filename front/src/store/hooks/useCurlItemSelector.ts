import { useCallback } from 'react';
import { useGetCurlItemByFileId, useAddCurlItem } from "./useCurl";
import { useSetCurrentFileId } from "./useFileexplorer";
import { getScriptDetails } from "../../api/getScriptsDetails";
import { TreeItem } from "react-complex-tree";
import { ItemData } from "../slices/fileexplorerSlice";

export const useCurlItemSelector = () => {
  const getCurlItem = useGetCurlItemByFileId();
  const addCurlItem = useAddCurlItem();
  const setCurrentFileId = useSetCurrentFileId();

  const selectCurlItem = useCallback(async (item: TreeItem<ItemData> | number) => {
    let fileId: number;
    let path: string;
    let name: string;

    if (typeof item === 'number') {
      fileId = item;
      const existingItem = getCurlItem(fileId);
      if (existingItem) {
        path = existingItem.script.url; // Assuming URL contains the path
        name = existingItem.script.url.split('/').pop() || ''; // Extracting name from URL
      } else {
        console.error('Item not found');
        return;
      }
    } else {
      if (item.isFolder) {
        return;
      }
      fileId = item.data.idx;
      path = item.data.path;
      name = item.data.name;
    }

    const existingCurlItem = getCurlItem(fileId);
    
    if (existingCurlItem) {
      // If the item already exists, just set it as current
      setCurrentFileId(fileId);
    } else {
      // If it doesn't exist, fetch the details and add it
      const scriptDetails = await getScriptDetails(path, name);
      if (scriptDetails) {
        addCurlItem({ fileId: fileId, script: scriptDetails.curlCommand }, fileId);
        setCurrentFileId(fileId);
      }
    }
  }, [getCurlItem, addCurlItem, setCurrentFileId]);

  return selectCurlItem;
};