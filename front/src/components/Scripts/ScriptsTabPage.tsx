import { TabGroup, TabList, TabPanels } from "@headlessui/react";
import TabHeader from "./TabHeader";
import TabContent from "./TabContent";

import { useMeasureHeaderHeight } from "./useMeasureHeaderHeight";
import { useSelectedIndex } from "./useSelectedIndex";

const ScritpsTabPage = () => {
  const { selectedIndex, setSelectedIndex, fileIds } = useSelectedIndex();
  const { tabListRef } = useMeasureHeaderHeight(fileIds);

  return (
    <div className="w-full h-full">
      <TabGroup className="w-full h-full flex flex-col" selectedIndex={selectedIndex} onChange={setSelectedIndex}>
        <TabList as="div" ref={tabListRef} className="flex flex-wrap">
          {fileIds.map((id) => {
            return <TabHeader idx={id} key={id} />;
          })}
        </TabList>
        <TabPanels className="h-full w-full p-2">
          {fileIds.map((id) => {
            return <TabContent idx={id} key={id} />;
          })}
        </TabPanels>
      </TabGroup>
    </div>
  );
};

export default ScritpsTabPage;
