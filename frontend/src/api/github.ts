import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export const githubApi = {
  // Repository endpoints
  getRepositories: async () => {
    const response = await axios.get(`${API_BASE_URL}/repositories`);
    return response.data;
  },

  // Notification endpoints
  getNotifications: async () => {
    const response = await axios.get(`${API_BASE_URL}/notifications`);
    return response.data;
  },

  // Metrics endpoints
  getMetrics: async (repoId: string) => {
    const response = await axios.get(`${API_BASE_URL}/metrics/${repoId}`);
    return response.data;
  },
};
