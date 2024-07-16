import apiClient from '../apiClient';
import { Response, checkError, ScriptDetails, ScriptDetailsCreate } from '../types';

/**
 * Get details of a script
 * @param path - The path of the script
 * @returns The details of the script
 */
export const getScriptDetails = async (path: string): Promise<ScriptDetails> => {
  const response = await apiClient.get<ScriptDetails>('/script', { path });
  return checkError(response);
};

/**
 * Create a new script
 * @param script - The details of the script to create
 * @returns The details of the created script
 */
export const createScript = async (script: ScriptDetailsCreate): Promise<ScriptDetails> => {
  const response = await apiClient.post<ScriptDetails>('/script', script);
  return checkError(response);
};

/**
 * Update an existing script
 * @param script - The details of the script to update
 * @returns The details of the updated script
 */
export const updateScript = async (script: ScriptDetailsCreate): Promise<ScriptDetails> => {
  const response = await apiClient.put<ScriptDetails>('/script', script);
  return checkError(response);
};

/**
 * Rename a script
 * @param oldFullName - The current full name of the script
 * @param newFullName - The new full name for the script
 * @returns The details of the renamed script
 */
export const renameScript = async (oldFullName: string, newFullName: string): Promise<ScriptDetails> => {
  const response = await apiClient.post<ScriptDetails>('/script/rename', { oldFullName, newFullName });
  return checkError(response);
};
