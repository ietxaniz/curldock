import { TabPanel } from "@headlessui/react";
import VerticalSplitPane from "./VerticalSplitPane";
import ScriptEditor from "../ScriptEditor";

const TabContent = ({ idx }: { idx: number }) => {
  return (
    <TabPanel className="outline outline-1 outline-gray-200 h-full w-full">
      <VerticalSplitPane
        minLeftWidth={10}
        maxLeftWidth={90}
        initial={50}
        left={
          <div className="h-full w-full overflow-auto">
            <ScriptEditor fileId={idx} />
          </div>
        }
        right={<div className="h-full w-full"></div>}
        leftOverflow={true}
        rightOverflow={true}
      />
    </TabPanel>
  );
};

export default TabContent;
