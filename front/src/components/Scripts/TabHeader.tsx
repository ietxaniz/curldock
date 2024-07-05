import { Tab } from "@headlessui/react";
import { useGetTreeData } from "../../store/hooks/useFileexplorer";
import { Fragment } from "react/jsx-runtime";
import clsx from "clsx";

const TabHeader = ({ idx }: { idx: number }) => {
  const treeData = useGetTreeData();
  const item = treeData[idx];
  const text = item.data.path + "/" + item.data.name;
  return (
    <Tab as={Fragment}>
      {({ hover, selected }) => <button className={clsx(hover && "underline", selected && "bg-blue-400 text-white", !selected && "outline outline-1 outline-gray-200", "m-1 p-1 rounded")}>{text}</button>}
    </Tab>
  );
};

export default TabHeader;
