import axios, { AxiosError, AxiosInstance } from 'axios';
import { Organization, Repository, User } from '../types/github'; // Assuming types are defined here

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  withCredentials: true,
});

// Attach JWT from localStorage for desktop mode
apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token');
  if (token) {
    config.headers = config.headers || {};
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response) {
      // Handle empty repository case
      if (error.response.data?.message?.includes('Git Repository is empty')) {
        return Promise.reject({
          message: 'This repository is empty. Analytics will be available once commits are added.',
          status: error.response.status,
          code: 'EMPTY_REPOSITORY',
          isEmptyRepo: true,
        });
      }

      // Handle 404 case
      if (error.response.status === 404) {
        return Promise.reject({
          message: 'Repository not found. Please check the repository name and try again.',
          status: 404,
          code: 'NOT_FOUND',
        });
      }

      // Handle other API errors
      return Promise.reject({
        message: error.response.data.message || 'An error occurred',
        status: error.response.status,
        code: error.response.data.code,
      });
    }

    if (error.request) {
      // Network error
      return Promise.reject({
        message: 'Unable to connect to the server. Please check your connection.',
        code: 'NETWORK_ERROR',
      });
    }

    // Unknown error
    return Promise.reject({
      message: error.message || 'An unexpected error occurred',
      code: 'UNKNOWN_ERROR',
    });
  },
);

export interface ApiError {
  message: string;
  status?: number;
  code?: string;
  isEmptyRepo?: boolean;
}

export interface Repository {
  id: number;
  name: string;
  owner: string;
  url: string;
  description?: string;
  language?: string;
  stars?: number;
  forks?: number;
  last_updated?: string;
}

export interface ActivityData {
  dates: string[];
  total_activity: number[];
  commits: number[];
}

export interface TrendData {
  dates: string[];
  commit_counts: number[];
}

export interface RepositoriesResponse {
  repositories: [string, string][]; // Array of [owner, name] tuples
  organizations: string[]; // Array of organization names
}

interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

interface ErrorResponse {
  message: string;
  status: number;
}

interface LoginResponse {
  redirectUrl: string;
}

interface UserResponse extends User {
  // Additional user response fields if needed
  accessToken?: string;
  refreshToken?: string;
}

class ApiService {
  private handleError(error: AxiosError): ApiError {
    if (error.response) {
      return {
        message: error.response.data?.message || 'An error occurred',
        status: error.response.status,
        code: error.response.data?.code,
      };
    }
    return {
      message: error.message || 'An error occurred',
      code: error.code,
    };
  }

  async listRepositories(): Promise<RepositoriesResponse> {
    try {
      const response = await apiClient.get('/repositories');
      if (!response.data.repositories || !Array.isArray(response.data.repositories)) {
        throw new Error('Invalid response format');
      }
      return response.data;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async getRepositoryDetails(id: number): Promise<Repository> {
    try {
      const response = await apiClient.get(`/repos/${id}`);
      return response.data;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async getRepositoryAnalytics(owner: string, repo: string): Promise<{ data: ActivityData }> {
    try {
      const response = await apiClient.get(`/analytics/repository/${owner}/${repo}`);
      return response;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async getActivityTrends(
    owner: string,
    repo: string,
    timeRange: string,
  ): Promise<{ data: ActivityData }> {
    try {
      const response = await apiClient.get(`/analytics/activity/${owner}/${repo}`, {
        params: { timeRange },
      });
      return response;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async addRepository(owner: string, repo: string): Promise<Repository> {
    try {
      const response = await apiClient.post(`/repos`, {
        owner,
        name: repo,
      });
      return response.data;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async removeRepository(id: number): Promise<void> {
    try {
      await apiClient.delete(`/repos/${id}`);
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async getRepositoryActivity(
    owner: string,
    repo: string,
    days: number = 30,
  ): Promise<{ data: ActivityData }> {
    try {
      const response = await apiClient.get(`/analytics/repository/${owner}/${repo}`, {
        params: { days },
      });
      return response;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async getRepositoryTrends(
    owner: string,
    repo: string,
    days: number = 30,
  ): Promise<{ data: TrendData }> {
    try {
      const response = await apiClient.get(`/analytics/trends/${owner}/${repo}`, {
        params: { days },
      });
      return response;
    } catch (error) {
      throw this.handleError(error as AxiosError);
    }
  }

  async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
    if (!response.ok) {
      const errorData: ErrorResponse = await response.json();
      throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
    }
    const data: T = await response.json();
    return {
      data,
      status: response.status,
      message: response.statusText,
    };
  }

  // --- Auth --- //
  async githubLogin(): Promise<LoginResponse> {
    const response = await apiClient.get<LoginResponse>('/auth/github');
    return response.data;
  }

  async logout(): Promise<void> {
    await apiClient.post('/auth/logout');
  }

  async getCurrentUser(): Promise<UserResponse> {
    const response = await apiClient.get<UserResponse>('/auth/me');
    return response.data;
  }

  // --- Organizations --- //
  async getOrganizations(): Promise<Organization[]> {
    const response = await apiClient.get<Organization[]>('/organizations');
    return response.data;
  }

  async syncOrganizations(): Promise<void> {
    await apiClient.post('/organizations/sync');
  }

  // --- Repositories --- //
  async getRepositories(): Promise<Repository[]> {
    const response = await apiClient.get<Repository[]>('/repositories');
    return response.data;
  }

  async syncRepositories(): Promise<void> {
    await apiClient.post('/repositories/sync');
  }
}

export const apiService = new ApiService();

export const fetchRepositoryMetrics = async (
  owner: string,
  repo: string,
): Promise<ApiResponse<Repository>> => {
  const response = await apiClient.get<Repository>(`/repositories/${owner}/${repo}/metrics`);
  return {
    data: response.data,
    status: response.status,
  };
};

export const fetchUserActivity = async (username: string): Promise<ApiResponse<User>> => {
  const response = await apiClient.get<User>(`/users/${username}/activity`);
  return {
    data: response.data,
    status: response.status,
  };
};
