import axios from "axios";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8080/api/v1";

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

export interface Task {
  id: number;
  repo_id: number;
  github_issue_id: number;
  title: string;
  priority: string;
  status: string;
  due_date: string;
}

export const taskService = {
  // List Tasks
  listTasks: (params?: { status?: string; priority?: string; repo?: string }) =>
    api.get("/tasks", { params }),

  // Create Task
  createTask: (data: {
    repo_id: number;
    title: string;
    priority: string;
    status: string;
    due_date: string;
  }) => api.post("/tasks", data),

  // Update Task
  updateTask: (id: number, data: {
    priority?: string;
    status?: string;
    due_date?: string;
  }) => api.put(`/tasks/${id}`, data),

  // Delete Task
  deleteTask: (id: number) => api.delete(`/tasks/${id}`),
};

export default taskService; 