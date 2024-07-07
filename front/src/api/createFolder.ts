import { Response, checkError, FolderCreate } from './base';

export const createFolder = async (folderDetails: FolderCreate): Promise<string | null> => {
  try {
    const response = await fetch('/api/v1/create-folder', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(folderDetails),
    });
    const data: Response<string> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
