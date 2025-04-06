import axios from "axios";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8080";

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

// Add request interceptor for authentication
api.interceptors.request.use((config) => {
  const token = localStorage.getItem("github_token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Add response interceptor for error handling
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Handle unauthorized access
      localStorage.removeItem("github_token");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  },
);

export const apiService = {
  // Repository Analytics
  getRepositoryActivity: (owner: string, repo: string) =>
    api.get(`/api/analytics/repository/${owner}/${repo}/activity`),

  getRepositoryHealth: (owner: string, repo: string) =>
    api.get(`/api/analytics/repository/${owner}/${repo}/health`),

  // User Analytics
  getUserContributions: (username: string) =>
    api.get(`/api/analytics/user/${username}/contributions`),

  // Organization Analytics
  getOrganizationStats: (org: string) =>
    api.get(`/api/analytics/organization/${org}/stats`),

  // Code Quality
  getCodeQualityMetrics: (owner: string, repo: string) =>
    api.get(`/api/analytics/repository/${owner}/${repo}/quality`),

  // Team Performance
  getTeamPerformance: (org: string) =>
    api.get(`/api/analytics/organization/${org}/team-performance`),

  // Authentication
  authenticate: (code: string) => api.post("/api/auth/github", { code }),

  // Data Sync
  syncRepository: (owner: string, repo: string) =>
    api.post(`/api/sync/repository/${owner}/${repo}`),

  syncOrganization: (org: string) => api.post(`/api/sync/organization/${org}`),
};

export default apiService;
