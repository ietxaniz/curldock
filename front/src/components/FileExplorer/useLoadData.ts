import { useEffect, useState } from "react";
import { useGetLoaded, useSetLoaded, useSetTreeData, useGetTreeData } from "@/store/hooks/useFileexplorer";
import { listAllFiles } from "@/api/endpoints/list";
import { TreeItemIndex } from "react-complex-tree";
import { FileType, getPathNameFromFullName } from "@/api/types";

export const useLoadData = () => {
  const isLoaded = useGetLoaded();
  const setLoaded = useSetLoaded();
  const setTreeData = useSetTreeData();
  const treeData = useGetTreeData();

  const [key, setKey] = useState(0);

  useEffect(() => {
    const initialize = async () => {
      if (!isLoaded) {
        const listAllFilesResponse = await listAllFiles();
        console.log(listAllFilesResponse);
        const serverData = listAllFilesResponse.files;
        setTreeData(serverData.map((info)=>{
          return {
            name: info.name,
            path: info.path,
            editing: false,
            itemType: info.fileType,
          }
        }));
        setLoaded(true);
      }
    };
    initialize();
  }, [isLoaded, setTreeData, setLoaded]);

  useEffect(() => {
    setKey((prevKey) => prevKey + 1);
    console.log("key updated");
  }, [treeData]);

  return { isLoaded, treeData, key };
}
