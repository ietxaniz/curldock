import apiClient from '../apiClient';
import { Response, checkError } from '../types';

/**
 * Delete a file or folder
 * @param path - The path of the file or folder to delete
 * @returns void
 */
export const deleteAsset = async (path: string): Promise<void> => {
  const response = await apiClient.delete<void>('/asset', { path });
  return checkError(response);
};
