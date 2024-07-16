import apiClient from '../apiClient';
import { Response, checkError, FileList } from '../types';

/**
 * List all files and folders recursively
 * @returns The list of all files and folders
 */
export const listAllFiles = async (): Promise<FileList> => {
  const response = await apiClient.get<FileList>('/list');
  return checkError(response);
};
