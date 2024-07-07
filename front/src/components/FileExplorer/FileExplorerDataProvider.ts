import { TreeDataProvider , TreeItem, TreeItemIndex} from "react-complex-tree";
import { ItemData } from "../../store/slices/fileexplorerSlice";

export class FileExplorerDataProvider implements TreeDataProvider<ItemData> {
  private _treeData: TreeItem<ItemData>[] = [];

  public setTreeData(treeData: TreeItem<ItemData>[]) {
    this._treeData = treeData;
  }

  public async getTreeItem(itemId: TreeItemIndex): Promise<TreeItem<ItemData>> {
    const numericItemId = typeof itemId === 'number' ? itemId : 0;
    return this._treeData[numericItemId];
}

  public async onRenameItem(item: TreeItem<ItemData>, _name: string) {
    console.log("rename ", item, _name);
  }
}
