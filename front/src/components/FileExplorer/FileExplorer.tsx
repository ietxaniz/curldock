import { UncontrolledTreeEnvironment, Tree, TreeItem } from "react-complex-tree";
import "react-complex-tree/lib/style-modern.css";
import { PlusIcon, FolderPlusIcon } from "@heroicons/react/24/solid";

import { useGetExpandedItems, useExpandItem, useCollapseItem, useRenameItem, useSetCurrentFileId, useAddFileToTree } from "../../store/hooks/useFileexplorer";

import { ItemData } from "../../store/slices/fileexplorerSlice";
import { FileExplorerDataProvider } from "./FileExplorerDataProvider";
import ItemTitleRenderer from "./ItemTitleRenderer";
import { getScriptDetails } from "../../api/getScriptsDetails";
import { useAddCurlItem } from "../../store/hooks/useCurl";
import { createScript } from "../../api/createScript";
import { createFolder } from "../../api/createFolder";
import { HttpMethod } from "@/api/base";
import { renameScript } from "../../api/renameScript";

import { useLoadData } from "./useLoadData";
import { useState } from "react";

const FileExplorer = () => {
  const expandedItems = useGetExpandedItems();
  const expandItem = useExpandItem();
  const collapseItem = useCollapseItem();
  const renameItem = useRenameItem();
  const addCurlItem = useAddCurlItem();
  const setCurrentFileId = useSetCurrentFileId();
  const addFileToTree = useAddFileToTree();
  const [currentPath, setCurrentPath] = useState("");

  const { isLoaded, treeData, key } = useLoadData();

  const onExpandItem = (item: TreeItem<ItemData>) => {
    expandItem(item.index as string);
  };

  const onCollapseItem = (item: TreeItem<ItemData>) => {
    collapseItem(item.index as string);
  };

  if (!isLoaded) {
    return <div>Loading...</div>;
  }

  const onRenameItem = async (item: TreeItem<ItemData>, newName: string) => {
    console.log("rename item", item, newName);

    // Call the backend API to rename the item
    const result = await renameScript({
      oldPath: item.data.path,
      newPath: item.data.path,
      oldName: item.data.name,
      newName: newName,
    });

    if (result) {
      renameItem(item, newName);
      if (item.isFolder) {
      }
    } else {
      console.error("Failed to rename item");
    }
  };

  const onFocusItem = async (item: TreeItem<ItemData>, id: string) => {
    if (item.isFolder) {
      setCurrentPath(item.data.path + (item.data.path.length > 0 ? "/" : "") + item.data.name);
      return;
    }
    setCurrentPath(item.data.path);
    const scriptDetails = await getScriptDetails(item.data.path, item.data.name);
    if (scriptDetails) {
      addCurlItem({ fileId: item.data.idx, script: scriptDetails.curlCommand }, item.data.idx);
      setCurrentFileId(item.data.idx);
    }
  };

  const handleCreateFile = async () => {
    const name = prompt("Enter the name of the new file:");
    if (name) {
      const result = await createScript({
        name,
        path: currentPath,
        curlCommand: { method: HttpMethod.GET, url: "http://localhost/", headers: [], cookies: [], options: {} },
      });
      if (result) {
        addFileToTree(result.name, result.path, false);
        addCurlItem({ fileId: treeData.length, script: result.curlCommand }, treeData.length);
      }
    }
  };

  const handleCreateFolder = async () => {
    const name = prompt("Enter the name of the new folder:");
    if (name) {
      const path = currentPath + (currentPath ? "/" : "") + name;
      const result = await createFolder({ path });
      if (result) {
        addFileToTree(name, currentPath, true);
      }
    }
  };

  const provider = new FileExplorerDataProvider();
  provider.setTreeData(treeData);

  return (
    <div className="h-full flex flex-col">
      <div className="toolbar flex items-center border-b border-gray-300">
        <button onClick={handleCreateFile} className="flex items-center space-x-1">
          <PlusIcon className="m-1 h-5 w-5 text-blue-500 hover:text-blue-700 active:text-blue-500" />
        </button>
        <button onClick={handleCreateFolder} className="flex items-center space-x-1">
          <FolderPlusIcon className="m-1 h-5 w-5 text-yellow-500 hover:text-yellow-700 active:text-yellow-500" />
        </button>
        <div className="px-2 text-sm text-center">{currentPath}</div>
      </div>
      <div className="flex-1 overflow-auto">
        <UncontrolledTreeEnvironment
          key={key}
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
    </div>
  );
};

export default FileExplorer;
