import { Response, checkError, ScriptList } from './base';

export const listScripts = async (path: string = ""): Promise<ScriptList | null> => {
  try {
    const response = await fetch(`/api/v1/scripts/${encodeURIComponent(path)}`);
    const data: Response<ScriptList> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
