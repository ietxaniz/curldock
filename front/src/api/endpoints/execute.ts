import apiClient from '../apiClient';
import { checkError, CurlCommand } from '../types';

/**
 * Execute a script
 * @param path - The path of the script to execute
 * @returns The result of the executed script
 */
export const executeScript = async (path: string): Promise<CurlCommand> => {
  const response = await apiClient.post<CurlCommand>('/execute', { path });
  return checkError(response);
};
