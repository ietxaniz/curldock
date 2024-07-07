import { Response, checkError, ScriptList, ScriptInfo } from './base';

export const getScriptList = async (): Promise<ScriptInfo[]> => {
  try {
    const response = await fetch("/api/v1/scrrecursive");
    const data: Response<ScriptList> = await response.json();
    const result = checkError(data);
    return result.scripts;
  } catch (error) {
    console.log(error);
    return [];
  }
}
