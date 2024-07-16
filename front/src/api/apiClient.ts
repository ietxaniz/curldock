import { Response } from "./types";

const BASE_URL = '/api/v1';

const apiClient = {
  get: async <T>(url: string, params?: Record<string, any>): Promise<Response<T>> => {
    const query = params ? `?${new URLSearchParams(params)}` : '';
    const response = await fetch(`${BASE_URL}${url}${query}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
    });
    return response.json();
  },

  post: async <T>(url: string, body: any): Promise<Response<T>> => {
    const response = await fetch(`${BASE_URL}${url}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    return response.json();
  },

  put: async <T>(url: string, body: any): Promise<Response<T>> => {
    const response = await fetch(`${BASE_URL}${url}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    return response.json();
  },

  delete: async <T>(url: string, params?: Record<string, any>): Promise<Response<T>> => {
    const query = params ? `?${new URLSearchParams(params)}` : '';
    const response = await fetch(`${BASE_URL}${url}${query}`, {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
    });
    return response.json();
  }
};

export default apiClient;
