import { useState, useEffect } from "react";
import { TreeItem } from "react-complex-tree";
import { ItemData } from "../../store/slices/fileexplorerSlice";
import { FileExplorerDataProvider } from "./FileExplorerDataProvider";
import { FolderIcon, DocumentIcon, TableCellsIcon, QuestionMarkCircleIcon } from "@heroicons/react/24/outline";
import { FileType } from "@/api/types";

const getFileIcon = (fileType: FileType) => {
  switch (fileType) {
    case FileType.Folder:
      return <FolderIcon className="h-5 w-5 text-gray-500" />;
    case FileType.Script:
      return <DocumentIcon className="h-5 w-5 text-gray-500" />;
    case FileType.Data:
      return <TableCellsIcon className="h-5 w-5 text-gray-500" />;
    default:
      return <QuestionMarkCircleIcon className="h-5 w-5 text-gray-500" />;
  }
};

const ItemTitleRenderer = ({ item, id, provider }: { item: TreeItem<ItemData>; id: number; provider: FileExplorerDataProvider }) => {
  const [name, setName] = useState(item.data.name);

  useEffect(() => {
    const getData = async () => {
      const treeItem = await provider.getTreeItem(item.data.idx);
      const newName = treeItem.data.name;
      if (name !== newName) {
        setName(newName);
        console.log(`update name from ${name} to ${newName}`);
      }
    };
    getData();
  }, [item, name, id, provider]);

  return (
    <div className="flex items-center space-x-2">
      {getFileIcon(item.data.itemType)}
      <span>{item.data.name}</span>
    </div>
  );
};

export default ItemTitleRenderer;
