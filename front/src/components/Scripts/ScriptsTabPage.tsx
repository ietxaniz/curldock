import { TabGroup, TabList, TabPanels } from "@headlessui/react";
import { useGetFileIds } from "../../store/hooks/useCurl";
import TabHeader from "./TabHeader";
import TabContent from "./TabContent";

const ScritpsTabPage = () => {
  const fileIds = useGetFileIds();
  return (
    <div className="w-full h-full">
      <TabGroup className="w-full h-full flex flex-col">
        <TabList>
          {fileIds.map((id) => {
            return <TabHeader idx={id} key={id}/>;
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
