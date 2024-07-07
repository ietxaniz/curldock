import { Tab } from "@headlessui/react";
import { useGetCurrentFileId, useGetTreeData, useSetCurrentFileId } from "../../store/hooks/useFileexplorer";
import { Fragment } from "react/jsx-runtime";
import clsx from "clsx";
import { useGetFileIds, useRemoveCurlItem } from "@/store/hooks/useCurl";

const TabHeader = ({ idx }: { idx: number }) => {
  const treeData = useGetTreeData();
  const item = treeData[idx];
  const text = item.data.path + "/" + item.data.name;

  const fileIds = useGetFileIds();
  const currentFileId = useGetCurrentFileId();
  const setCurrentFileId = useSetCurrentFileId();
  const removeCurlItem = useRemoveCurlItem();

  const handleCloseClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    console.log(currentFileId, idx);
    if (currentFileId === idx) {
      if (fileIds.indexOf(idx) > 0) {
        console.log(`index: ${fileIds.indexOf(idx)}, next: ${fileIds[fileIds.indexOf(idx)-1]}`)
        setCurrentFileId(fileIds[fileIds.indexOf(idx)-1]);
      } else {
        if (fileIds.length > 1) {
          setCurrentFileId(fileIds[1]);
        }
      }
    }
    removeCurlItem(idx);
  };

  return (
    <Tab as={Fragment}>
      {({ hover, selected }) => (
        <div className={clsx(
          hover && "underline",
          selected && "bg-blue-400 text-white",
          !selected && "outline outline-1 outline-gray-200",
          "m-1 p-1 rounded flex whitespace-nowrap"
        )}>
          <button>
            {text}
          </button>
          <button onClick={handleCloseClick} className="ml-2 p-1 text-gray-500 hover:text-gray-700">
            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      )}
    </Tab>
  );
};

export default TabHeader;
