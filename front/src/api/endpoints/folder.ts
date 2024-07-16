import apiClient from '../apiClient';
import { Response, checkError } from '../types';

/**
 * Create a new folder
 * @param path - The path of the folder to create
 * @returns void
 */
export const createFolder = async (path: string): Promise<void> => {
  const response = await apiClient.post<void>('/folder', { path });
  return checkError(response);
};
