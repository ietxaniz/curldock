import { useAppDispatch, useAppSelector } from "@/store/hooks/hooks";
import {
  setScript,
  renameScript,
  removeScript,
} from "@/store/slices/scriptInputSlice";
import { ScriptDetails } from "@/api/types";

export const useGetScripts = ():{[key: string]: ScriptDetails} => {
  return useAppSelector((state) => state.scriptInput);
};

export const useGetScript = (fullName: string): ScriptDetails | undefined => {
  return useAppSelector((state) => state.scriptInput[fullName]);
};

export const useSetScript = (): ((script: ScriptDetails) => void) => {
  const dispatch = useAppDispatch();
  return (script: ScriptDetails) => {
    dispatch(setScript(script));
  };
};

export const useRenameScript = (): ((oldName: string, newName: string) => void) => {
  const dispatch = useAppDispatch();
  return (oldName: string, newName: string) => {
    dispatch(renameScript({ oldName, newName }));
  };
};

export const useRemoveScript = (): ((fullName: string) => void) => {
  const dispatch = useAppDispatch();
  return (fullName: string) => {
    dispatch(removeScript(fullName));
  };
};
