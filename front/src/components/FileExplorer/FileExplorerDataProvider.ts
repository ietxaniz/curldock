import { TreeDataProvider, TreeItem, TreeItemIndex, Disposable } from "react-complex-tree";
import { ItemData } from "../../store/slices/fileexplorerSlice";

export class FileExplorerDataProvider implements TreeDataProvider<ItemData> {
  static _instance: FileExplorerDataProvider | null = null;
  
  public static getInstance(): FileExplorerDataProvider {
    if (!this._instance) {
      this._instance = new FileExplorerDataProvider();
    }
    return this._instance;
  }

  private _treeData: Record<TreeItemIndex, TreeItem<ItemData>> = {};
  private _changeListeners: ((changedItemIds: TreeItemIndex[]) => void)[] = [];
  private _rootItem: TreeItemIndex = 'root';

  public setTreeData(newTreeData: TreeItem<ItemData>[]) {
    this._treeData = newTreeData.reduce((acc, item) => {
      acc[item.index] = item;
      return acc;
    }, {} as Record<TreeItemIndex, TreeItem<ItemData>>);

    // Notify about all items
    const allItemIds = Object.keys(this._treeData);
    allItemIds.push('0');
    console.log("Notifying changes for items:", allItemIds);
    this.notifyListeners(allItemIds);
  }

  public async getTreeItem(itemId: TreeItemIndex): Promise<TreeItem<ItemData>> {
    return this._treeData[itemId];
  }

  public onDidChangeTreeData(listener: (changedItemIds: TreeItemIndex[]) => void): Disposable {
    console.log(`add listener ${listener}`)
    this._changeListeners.push(listener);
    return {
      dispose: () => {
        console.log("call to dispose")
        const index = this._changeListeners.indexOf(listener);
        if (index > -1) {
          this._changeListeners.splice(index, 1);
        }
      }
    };
  }

  public async onRenameItem(item: TreeItem<ItemData>, newName: string): Promise<void> {
    console.log("Renaming item:", item.index, "to", newName);
    this.notifyListeners([item.index]);
  }

  public async onChangeItemChildren(itemId: TreeItemIndex, newChildren: TreeItemIndex[]): Promise<void> {
    console.log("Changing children of item:", itemId, "to", newChildren);
    this.notifyListeners([itemId, ...newChildren]);
  }

  private notifyListeners(changedItemIds: TreeItemIndex[]) {
    this._changeListeners.forEach(listener => listener(['root']));
  }

  public getRootItem(): TreeItemIndex {
    return this._rootItem;
  }
}