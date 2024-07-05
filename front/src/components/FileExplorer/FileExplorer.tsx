import { useEffect, useRef, useState } from "react";
import { UncontrolledTreeEnvironment, Tree, TreeItem, TreeItemIndex } from "react-complex-tree";
import "react-complex-tree/lib/style-modern.css";

import {
  useGetLoaded,
  useSetLoaded,
  useGetTreeData,
  useSetTreeData,
  useGetExpandedItems,
  useExpandItem,
  useCollapseItem,
  useRenameItem,
} from "../../store/hooks/useFileexplorer";

import { ItemData } from "../../store/slices/fileexplorerSlice";
import { FileExplorerDataProvider } from "./FileExplorerDataProvider";
import ItemTitleRenderer from "./ItemTitleRenderer";
import { getScriptsInfo } from '../../api/script';
import { getScriptsDetails } from "../../api/scriptDetail";
import { useAddCurlItem } from "../../store/hooks/useCurl";

const FileExplorer = () => {
  const isLoaded = useGetLoaded();
  const setLoaded = useSetLoaded();
  const treeData = useGetTreeData();
  const setTreeData = useSetTreeData();
  const expandedItems = useGetExpandedItems();
  const expandItem = useExpandItem();
  const collapseItem = useCollapseItem();
  const renameItem = useRenameItem();
  const [key, setKey] = useState(0);
  const mainDivRef = useRef<HTMLDivElement>(null);
  const addCurlItem = useAddCurlItem();

  useEffect(() => {
    setKey(key + 1);
  }, [treeData]);

  useEffect(() => {
    const initialize = async () => {
      if (!isLoaded) {
        const serverData = await getScriptsInfo();
        let folders: { [key: string]: number } = {};
        folders[""] = 0;
        const initialData = [{
          index: "root" as TreeItemIndex,
          isFolder: true,
          children: [] as number[],
          data: { name: "Root", editing: false, idx: 0, path: "" },
        }];
        for (let i=0; i<serverData.length;i++) {
          const idx = i + 1;
          const parentidx = folders[serverData[i].path];
          initialData[parentidx].children.push(idx);
          if (serverData[i].is_folder) {
            const leftPathPath = serverData[i].path.length > 0 ? serverData[i].path + "/" : "";
            const path = leftPathPath + serverData[i].name;
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: true,
              data: { name: serverData[i].name, editing: false, idx: idx, path: leftPathPath}
            });
            folders[path] = idx;
          } else {
            initialData.push({
              index: idx,
              children: [] as number[],
              isFolder: false,
              data: { name: serverData[i].name, editing: false, idx: idx, path: serverData[i].path}
            });
          }
        }
        setTreeData(initialData);
        setLoaded(true);
      }
    }
    initialize();
    
  }, [isLoaded, setTreeData, setLoaded]);

  const onExpandItem = (item: TreeItem<ItemData>) => {
    expandItem(item.index as string);
  };

  const onCollapseItem = (item: TreeItem<ItemData>) => {
    collapseItem(item.index as string);
  };

  if (!isLoaded) {
    return <div>Loading...</div>;
  }

  const onRenameItem = (item: TreeItem<ItemData>, name: string) => {
    console.log("rename item", item, name);
    renameItem(item, name);
  };

  const onFocusItem = async (item: TreeItem<ItemData>, id: string) => {
    if (item.isFolder) {
      return;
    }
    const path = item.data.path + "/" + item.data.name;
    console.log(path);
    console.log(item);
    console.log(id);
    const scriptDetails = await getScriptsDetails(item.data.path + "/" + item.data.name);
    if (scriptDetails) {
      addCurlItem({fileId: item.data.idx, script: scriptDetails}, item.data.idx);
    }
  };

  const provider = new FileExplorerDataProvider();
  provider.setTreeData(treeData);

  return (
    <div className="h-full overflow-auto" ref={mainDivRef}>
      <UncontrolledTreeEnvironment
        dataProvider={provider}
        getItemTitle={(item: TreeItem<ItemData>) => item.data.name}
        viewState={{ "tree-1": { expandedItems } }}
        onExpandItem={onExpandItem}
        onCollapseItem={onCollapseItem}
        onRenameItem={onRenameItem}
        onFocusItem={onFocusItem}
        renderItemTitle={(props: { item: TreeItem<ItemData> }) => {
          return <ItemTitleRenderer item={props.item} id={key} provider={provider} />;
        }}
      >
        <Tree treeId="tree-1" rootItem="root" treeLabel="File Explorer" />
      </UncontrolledTreeEnvironment>
    </div>
  );
};

export default FileExplorer;
