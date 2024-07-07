import { Response, checkError, ScriptList } from './base';

export const listScriptsRecursive = async (): Promise<ScriptList | null> => {
  try {
    const response = await fetch("/api/v1/scrrecursive");
    const data: Response<ScriptList> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
