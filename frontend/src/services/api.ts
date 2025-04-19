import axios, { AxiosInstance } from "axios";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8080";

const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: { "Content-Type": "application/json" },
  withCredentials: true,
});

// Attach JWT from localStorage for desktop mode
apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem("auth_token");
  if (token && config.headers) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export interface PatResponse {
  jwt: string;
}

export const patAuth = async (pat: string): Promise<PatResponse> => {
  const resp = await apiClient.post<PatResponse>("/auth/pat", { pat });
  return resp.data;
};

export const logout = async (): Promise<void> => {
  await apiClient.post("/auth/logout");
};
