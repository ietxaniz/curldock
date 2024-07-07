import { useState, useEffect } from 'react';
import { TreeItem } from 'react-complex-tree';
import { ItemData } from '../../store/slices/fileexplorerSlice';
import { FileExplorerDataProvider } from './FileExplorerDataProvider';

const ItemTitleRenderer = ({item, id, provider}:{item: TreeItem<ItemData>,id:number, provider: FileExplorerDataProvider}) => {
  const [name, setName] = useState(item.data.name);

  useEffect(() => {
    const getData = async () => {
      const treeItem = await provider.getTreeItem(item.data.idx);
      const newName = treeItem.data.name;
      if (name !== newName) {
        setName(newName);
        console.log(`update name from ${name} to ${newName}`)
      }
    }
    getData();
  }, [item, name, id, provider])
  return (
    <div>{name}</div>
  )
}

export default ItemTitleRenderer