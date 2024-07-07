import { Response, checkError, ScriptDetails } from './base';

interface RenameScriptParams {
  old_name: string;
  new_name: string;
  old_path: string;
  new_path: string;
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