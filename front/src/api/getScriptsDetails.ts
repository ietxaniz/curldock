import { Response, checkError, ScriptDetails } from './base';

export const getScriptDetails = async (path: string, name: string): Promise<ScriptDetails | null> => {
  try {
    const response = await fetch(`/api/v1/script-details/${path}/${name}`);
    const data: Response<ScriptDetails> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
