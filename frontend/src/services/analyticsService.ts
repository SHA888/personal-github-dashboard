import axios from "axios";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8000/api";

export interface RepositoryAnalytics {
  total_commits: number;
  total_contributors: number;
  commit_history: {
    date: string;
    count: number;
  }[];
}

export interface ActivityTrends {
  dates: string[];
  commit_counts: number[];
}

class AnalyticsService {
  async getRepositoryAnalytics(
    owner: string,
    repo: string,
    timeRange: string,
  ): Promise<{ data: RepositoryAnalytics }> {
    const response = await axios.get(
      `${API_BASE_URL}/analytics/repos/${owner}/${repo}`,
      {
        params: { timeRange },
      },
    );
    return response;
  }

  async getActivityTrends(
    owner: string,
    repo: string,
    timeRange: string,
  ): Promise<{ data: ActivityTrends }> {
    const response = await axios.get(
      `${API_BASE_URL}/analytics/activity/${owner}/${repo}`,
      {
        params: { timeRange },
      },
    );
    return response;
  }
}

export const analyticsService = new AnalyticsService();
