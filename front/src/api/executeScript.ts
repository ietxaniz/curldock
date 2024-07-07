import { Response, checkError, CurlCommandResult } from './base';

export const executeScript = async (path: string, name: string): Promise<CurlCommandResult | null> => {
  try {
    const response = await fetch(`/api/v1/execute/${encodeURIComponent(path)}/${encodeURIComponent(name)}`, {
      method: 'POST',
    });
    const data: Response<CurlCommandResult> = await response.json();
    return checkError(data);
  } catch (error) {
    console.error(error);
    return null;
  }
};
