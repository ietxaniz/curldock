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
        const folders: { [key: string]: number } = {};
        folders[""] = 0;
        const initialData = [
          {
            index: "root" as TreeItemIndex,
            isFolder: true,
            children: [] as number[],
            data: { name: "Root", editing: false, idx: 0, path: "", itemType: FileType.Data },
          },
        ];
        for (let i = 0; i < serverData.length; i++) {
          const idx = i + 1;
          const { path } = getPathNameFromFullName(serverData[i].path);
          const parentidx = folders[path];
          initialData[parentidx].children.push(idx);
          if (serverData[i].isFolder) {
            const fullPath = serverData[i].path;
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: true,
              data: { name: serverData[i].name, editing: false, idx: idx, path, itemType: serverData[i].fileType },
            });
            folders[fullPath] = idx;
          } else {
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: false,
              data: { name: serverData[i].name, editing: false, idx: idx, path, itemType: serverData[i].fileType },
            });
          }
        }
        setTreeData(initialData);
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
