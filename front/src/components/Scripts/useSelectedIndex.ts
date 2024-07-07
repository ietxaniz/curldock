import { useEffect, useRef, useState } from "react";
import { useGetFileIds } from "@/store/hooks/useCurl";
import { useGetCurrentFileId, useSetCurrentFileId } from "@/store/hooks/useFileexplorer";

export const useSelectedIndex = () => {
  const [selectedIndex, setSelectedIndex] = useState(0);
  const selectedIndexRef = useRef(selectedIndex);
  selectedIndexRef.current = selectedIndex;
  const fileIds = useGetFileIds();
  const fileIdsRef = useRef(fileIds);
  fileIdsRef.current = fileIds;
  const currentFileId = useGetCurrentFileId();
  const setCurrentFileId = useSetCurrentFileId();
  const setCurrentFileIdRef = useRef(setCurrentFileId);
  setCurrentFileIdRef.current = setCurrentFileId;
  const currentFileIdIndex = fileIds.indexOf(currentFileId);
  const currentFileIdIndexRef = useRef(currentFileIdIndex);
  currentFileIdIndexRef.current = currentFileIdIndex;

  useEffect(() => {
    const currentFileIdIndex = currentFileIdIndexRef.current;
    if (currentFileIdIndex !== selectedIndex) {
      const fileIds = fileIdsRef.current;
      if (fileIds.length > selectedIndex && selectedIndex >= 0) {
        const setCurrentFileId = setCurrentFileIdRef.current;
        setCurrentFileId(fileIds[selectedIndex]);
      }
    }
  }, [selectedIndex]);

  useEffect(()=> {
    const selectedIndex = selectedIndexRef.current;
    if (currentFileIdIndex !== selectedIndex) {
      if (currentFileIdIndex >= 0) {
        setSelectedIndex(currentFileIdIndex);
      }
    }
  }, [currentFileIdIndex, currentFileId]);

  return {selectedIndex, setSelectedIndex, fileIds};
}

