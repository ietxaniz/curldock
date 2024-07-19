import { useAppDispatch, useAppSelector } from "@/store/hooks/hooks";
import {
  addResult,
  renameResult,
  removeAllResultsOfScript,
  removeSpecificResult,
} from "@/store/slices/scriptResultSlice";
import { CurlCommandResult } from "@/api/types";

export const useGetScriptResults = ():{[key: string]: CurlCommandResult[]} => {
  return useAppSelector((state) => state.scriptResult);
};

export const useAddResult = (): ((name: string, result: CurlCommandResult) => void) => {
  const dispatch = useAppDispatch();
  return (name: string, result: CurlCommandResult) => {
    dispatch(addResult({ name, result }));
  };
};

export const useRenameResult = (): ((oldName: string, newName: string) => void) => {
  const dispatch = useAppDispatch();
  return (oldName: string, newName: string) => {
    dispatch(renameResult({ oldName, newName }));
  };
};

export const useRemoveAllResultsOfScript = (): ((name: string) => void) => {
  const dispatch = useAppDispatch();
  return (name: string) => {
    dispatch(removeAllResultsOfScript(name));
  };
};

export const useRemoveSpecificResult = (): ((name: string, index: number) => void) => {
  const dispatch = useAppDispatch();
  return (name: string, index: number) => {
    dispatch(removeSpecificResult({ name, index }));
  };
};
