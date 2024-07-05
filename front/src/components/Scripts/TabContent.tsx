import { TabPanel } from "@headlessui/react";
import { useGetTreeData } from "../../store/hooks/useFileexplorer";
import VerticalSplitPane from "./VerticalSplitPane";
import ScriptEditor from "../ScriptEditor";

const TabContent = ({ idx }: { idx: number }) => {
  const treeData = useGetTreeData();
  const item = treeData[idx];
  const text = item.data.path + "/" + item.data.name;
  console.log(text);
  return (
    <TabPanel className="outline outline-1 outline-gray-200 h-full w-full overflow-aut">
      <VerticalSplitPane
        minLeftWidth={10}
        maxLeftWidth={90}
        initial={50}
        left={<div className="h-full w-full overflow-auto"><ScriptEditor/></div>}
        right={<div className="h-full w-full"></div>}
        leftOverflow={true}
        rightOverflow={true}
      />
    </TabPanel>
  )
}

export default TabContent