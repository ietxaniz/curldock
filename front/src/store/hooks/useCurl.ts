import { useAppDispatch, useAppSelector } from "./hooks";
import { addCurlItem, updateCurlItem, removeCurlItem, Curl, updateCurlResult, CurlCommandResult } from "../slices/curlSlice";

export const useGetCurlItems = () => {
  return useAppSelector(state => state.curl.curlItems);
};

export const useGetFileIds = () => {
  return useAppSelector(state => state.curl.fileIds);
};

export const useAddCurlItem = (): ((curl: Curl, fileId: number) => void) => {
  const dispatch = useAppDispatch();
  return (curl: Curl, fileId: number) => {
    dispatch(addCurlItem({ curl, fileId }));
  };
};

export const useUpdateCurlItem = (): ((curl: Curl) => void) => {
  const dispatch = useAppDispatch();
  return (curl: Curl) => {
    dispatch(updateCurlItem(curl));
  };
};

export const useRemoveCurlItem = (): ((fileId: number) => void) => {
  const dispatch = useAppDispatch();
  return (fileId: number) => {
    dispatch(removeCurlItem(fileId));
  };
};

export const useGetCurlItemByFileId = (): ((fileId: number) => Curl | undefined) => {
  const curlItems = useGetCurlItems();
  return (fileId: number) => {
    return curlItems.find(item => item.fileId === fileId);
  };
};

export const useUpdateCurlResult = (): ((fileId: number, result: CurlCommandResult) => void) => {
  const dispatch = useAppDispatch();
  return (fileId: number, result: CurlCommandResult) => {
    dispatch(updateCurlResult({ fileId, result }));
  };
};
