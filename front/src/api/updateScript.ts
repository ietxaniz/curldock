import { Response, checkError, ScriptDetails, ScriptDetailsCreate } from './base';

export const updateScript = async (scriptDetails: ScriptDetailsCreate): Promise<ScriptDetails | null> => {
  try {
    const response = await fetch('/api/v1/script', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(scriptDetails),
    });
    const data: Response<ScriptDetails> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
