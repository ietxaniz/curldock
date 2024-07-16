// src/api/endpoints/data.ts

import apiClient from '../apiClient';
import { Response, checkError, DataFileDetails } from '../types';

/**
 * Load a data file
 * @param path - The path of the data file to load
 * @returns The details of the loaded data file
 */
export const loadDataFile = async (path: string): Promise<DataFileDetails> => {
  const response = await apiClient.get<DataFileDetails>('/data', { path });
  return checkError(response);
};

/**
 * Store a new data file
 * @param path - The path where the data file will be stored
 * @param content - The content of the data file
 * @returns The details of the stored data file
 */
export const storeDataFile = async (path: string, content: Record<string, string>): Promise<DataFileDetails> => {
  const response = await apiClient.post<DataFileDetails>('/data', { path, content });
  return checkError(response);
};

/**
 * Update an existing data file
 * @param path - The path of the data file to update
 * @param content - The new content of the data file
 * @returns The details of the updated data file
 */
export const updateDataFile = async (path: string, content: Record<string, string>): Promise<DataFileDetails> => {
  const response = await apiClient.put<DataFileDetails>('/data', { path, content });
  return checkError(response);
};

/**
 * Delete a data file
 * @param path - The path of the data file to delete
 * @returns void
 */
export const deleteDataFile = async (path: string): Promise<void> => {
  const response = await apiClient.delete<void>('/asset', { path });
  return checkError(response);
};

/**
 * Rename a data file
 * @param oldPath - The current path of the data file
 * @param newPath - The new path for the data file
 * @returns void
 */
export const renameDataFile = async (oldPath: string, newPath: string): Promise<void> => {
  const response = await apiClient.post<void>('/data/rename', { oldPath, newPath });
  return checkError(response);
};
