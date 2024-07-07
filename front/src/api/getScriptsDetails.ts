import { Response, checkError } from './base';
import { ScriptDetails } from './base';

export const getScriptsDetails = async (path: string, name: string): Promise<ScriptDetails | null> => {
  try {
    const response = await fetch(`/api/v1/script-details/${path}/${name}`);
    const data: Response<ScriptDetails> = await response.json();
    const scriptDetails = checkError(data);
    return scriptDetails;
  } catch (error) {
    console.log(error);
    return null;
  }
}
