import { Response, checkError, ScriptDetails } from './base';

interface RenameScriptParams {
  oldPath: string;
  newPath: string;
  oldName: string;
  newName: string;
}

export const renameScript = async (params: RenameScriptParams): Promise<ScriptDetails | null> => {
  try {
    const response = await fetch('/api/v1/rename-script', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(params),
    });
    const data: Response<ScriptDetails> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};