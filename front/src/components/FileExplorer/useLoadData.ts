import { useEffect, useState } from "react";
import { useGetLoaded, useSetLoaded, useSetTreeData, useGetTreeData } from "@/store/hooks/useFileexplorer";
import { getScriptList } from "@/api/getScriptList";
import { TreeItemIndex } from "react-complex-tree";

export const useLoadData = () => {
  const isLoaded = useGetLoaded();
  const setLoaded = useSetLoaded();
  const setTreeData = useSetTreeData();
  const treeData = useGetTreeData();

  const [key, setKey] = useState(0);

  useEffect(() => {
    const initialize = async () => {
      if (!isLoaded) {
        const serverData = await getScriptList();
        const folders: { [key: string]: number } = {};
        folders[""] = 0;
        const initialData = [
          {
            index: "root" as TreeItemIndex,
            isFolder: true,
            children: [] as number[],
            data: { name: "Root", editing: false, idx: 0, path: "" },
          },
        ];
        for (let i = 0; i < serverData.length; i++) {
          const idx = i + 1;
          const parentidx = folders[serverData[i].path];
          initialData[parentidx].children.push(idx);
          if (serverData[i].isFolder) {
            const leftPathPath = serverData[i].path.length > 0 ? serverData[i].path + "/" : "";
            const path = leftPathPath + serverData[i].name;
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: true,
              data: { name: serverData[i].name, editing: false, idx: idx, path: leftPathPath },
            });
            folders[path] = idx;
          } else {
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: false,
              data: { name: serverData[i].name, editing: false, idx: idx, path: serverData[i].path },
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
