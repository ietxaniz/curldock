import { useEffect, useRef, useCallback } from "react";
import { useSetHeaderHeight } from "@/store/hooks/useLayout";

export const useMeasureHeaderHeight = (fileIds:number[]) => {
  const tabListRef = useRef<HTMLDivElement>(null);
  const setHeaderHeight = useSetHeaderHeight();

  const updateHeaderHeight = useCallback(() => {
    if (tabListRef.current) {
      setHeaderHeight(tabListRef.current.offsetHeight);
    }
  }, [setHeaderHeight]);

  useEffect(() => {
    updateHeaderHeight();
  }, [fileIds, updateHeaderHeight]);

  useEffect(() => {
    window.addEventListener("resize", updateHeaderHeight);
    return () => {
      window.removeEventListener("resize", updateHeaderHeight);
    };
  }, [updateHeaderHeight]);

  return { tabListRef };
}
