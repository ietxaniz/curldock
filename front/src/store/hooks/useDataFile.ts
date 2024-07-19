import { useAppDispatch, useAppSelector } from "@/store/hooks/hooks";
import {
  addDataFile,
  updateDataFileContent,
  removeDataFile,
  renameDataFile,
  reorderDataFiles,
} from "@/store/slices/dataFileSlice";
import { DataFileDetails } from "@/api/types";

export const useGetDataFiles = (): DataFileDetails[] => {
  return useAppSelector((state) => state.dataFile.files);
};

export const useAddDataFile = (): ((file: DataFileDetails) => void) => {
  const dispatch = useAppDispatch();
  return (file: DataFileDetails) => {
    dispatch(addDataFile(file));
  };
};

export const useUpdateDataFileContent = (): ((fullName: string, content: Record<string, string>, createdAt: number, updatedAt: number) => void) => {
  const dispatch = useAppDispatch();
  return (fullName: string, content: Record<string, string>, createdAt: number, updatedAt: number) => {
    dispatch(updateDataFileContent({ fullName, content, createdAt, updatedAt }));
  };
};

export const useRemoveDataFile = (): ((fullName: string) => void) => {
  const dispatch = useAppDispatch();
  return (fullName: string) => {
    dispatch(removeDataFile(fullName));
  };
};

export const useRenameDataFile = (): ((fullName: string, newFullName: string) => void) => {
  const dispatch = useAppDispatch();
  return (fullName: string, newFullName: string) => {
    dispatch(renameDataFile({ fullName, newFullName }));
  };
};

export const useReorderDataFiles = (): (() => void) => {
  const dispatch = useAppDispatch();
  return () => {
    dispatch(reorderDataFiles(null));
  };
};
