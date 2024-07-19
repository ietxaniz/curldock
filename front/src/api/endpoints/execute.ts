import apiClient from '../apiClient';
import { checkError, CurlCommand, CurlCommandResult } from '../types';

/**
 * Execute a script
 * @param path - The path of the script to execute
 * @returns The result of the executed script
 */
export const executeScript = async (path: string): Promise<CurlCommandResult> => {
  const response = await apiClient.post<CurlCommandResult>('/execute', { path });
  return checkError(response);
};
