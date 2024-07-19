import { UncontrolledTreeEnvironment, Tree, TreeItem, TreeItemIndex } from "react-complex-tree";
import "react-complex-tree/lib/style-modern.css";
import { PlusIcon, FolderPlusIcon, HomeIcon, TableCellsIcon } from "@heroicons/react/24/solid";

import { useGetExpandedItems, useExpandItem, useCollapseItem, useRenameItem, useAddFileToTree } from "../../store/hooks/useFileexplorer";
import { useCurlItemSelector } from "@/store/hooks/useCurlItemSelector";

import { ItemData } from "../../store/slices/fileexplorerSlice";
import { FileExplorerDataProvider } from "./FileExplorerDataProvider";
import ItemTitleRenderer from "./ItemTitleRenderer";
import { useAddCurlItem } from "../../store/hooks/useCurl";
import { createScript } from "@/api/endpoints/script";
import { createFolder } from "@/api/createFolder";
import { storeDataFile } from "@/api/endpoints/data";
import { HttpMethod, getPathNameFromFullName } from "@/api/types";
import { renameScript } from "../../api/renameScript";

import { useLoadData } from "./useLoadData";
import { useState } from "react";
import { FileType } from "@/api/types";

const FileExplorer = () => {
  const expandedItems = useGetExpandedItems();
  const expandItem = useExpandItem();
  const collapseItem = useCollapseItem();
  const renameItem = useRenameItem();
  const addCurlItem = useAddCurlItem();
  const addFileToTree = useAddFileToTree();
  const [currentPath, setCurrentPath] = useState("");
  const selectCurlItem = useCurlItemSelector();

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

  const onClickItem = async (item: TreeItem<ItemData>, id: string) => {
    if (item.isFolder) {
      setCurrentPath(item.data.path + (item.data.path.length > 0 ? "/" : "") + item.data.name);
      return;
    }
    setCurrentPath(item.data.path);
    await selectCurlItem(item);
  };

  const onSelectItems = async (itemIndexes: TreeItemIndex[], treeId: string) => {
    if (treeId !== "tree-1") {
      return;
    }

    for (const index of itemIndexes) {
      const item = treeData.find((item) => item.index === index);
      if (item) {
        await onClickItem(item, treeId);
      }
    }
  };

  const handleCreateFile = async () => {
    const name = prompt("Enter the name of the new file:");
    if (name) {
      let fullName = name;
      if (currentPath.length > 0) {
        fullName = currentPath + "/" + name;
      }
      const result = await createScript({
        fullName,
        curlCommand: {
          method: HttpMethod.GET,
          url: "http://localhost/",
          headers: [],
          cookies: [],
          options: {},
          storeCurlBody: [],
          storeCurlCookie: [],
          loadCurl: [],
        },
      });
      if (result) {
        const { path, name } = getPathNameFromFullName(result.fullName);
        addFileToTree(name, path, FileType.Script);
        // TODO: We will add curl item later. addCurlItem({ fileId: treeData.length, script: result.curlCommand }, treeData.length);
      }
    }
  };

  const handleCreateFolder = async () => {
    const name = prompt("Enter the name of the new folder:");
    if (name) {
      const path = currentPath + (currentPath ? "/" : "") + name;
      const result = await createFolder({ path });
      if (result) {
        addFileToTree(name, currentPath, FileType.Folder);
      }
    }
  };

  const handleCreateDataFile = async () => {
    // TODO: create data file
  };

  const onHome = () => {
    setCurrentPath("");
  };

  // const provider = FileExplorerDataProvider.getInstance();
  const provider = new FileExplorerDataProvider();
  provider.setTreeData(treeData);

  console.log(key, treeData);

  return (
    <div className="h-full flex flex-col">
      <div className="toolbar flex items-center border-b border-gray-300">
        <button onClick={onHome} className="flex items-center space-x-1">
          <HomeIcon className="m-1 h-5 w-5 text-gray-300 hover:text-gray-500 active:text-gray-300" />
        </button>
        <button onClick={handleCreateFile} className="flex items-center space-x-1">
          <PlusIcon className="m-1 h-5 w-5 text-gray-300 hover:text-gray-500 active:text-gray-300" />
        </button>
        <button onClick={handleCreateDataFile} className="flex items-center space-x-1">
          <TableCellsIcon className="m-1 h-5 w-5 text-gray-300 hover:text-gray-500 active:text-gray-300" />
        </button>
        <button onClick={handleCreateFolder} className="flex items-center space-x-1">
          <FolderPlusIcon className="m-1 h-5 w-5 text-gray-300 hover:text-gray-500 active:text-gray-300" />
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
          onSelectItems={onSelectItems}
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
