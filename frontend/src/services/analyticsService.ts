import axios from "axios";

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || "http://localhost:3000/api";

export interface AnalyticsResponse {
    data: {
        commit_stats: {
            total: number;
            by_author: Record<string, number>;
        };
        pull_request_stats: {
            open: number;
            closed: number;
            merged: number;
        };
        issue_stats: {
            open: number;
            closed: number;
        };
        commit_activity: {
            daily: number[];
            weekly: number[];
            monthly: number[];
        };
    };
}

export const analyticsService = {
    getRepositoryAnalytics: async (
        owner: string,
        repo: string,
        timeRange: string = "30d"
    ): Promise<AnalyticsResponse> => {
        const response = await axios.get(
            `${API_BASE_URL}/analytics/repositories/${owner}/${repo}`,
            {
                params: { timeRange },
            }
        );
        return response.data;
    },

    getUserAnalytics: (username: string) =>
        axios.get(`${API_BASE_URL}/analytics/users/${username}`),

    // Activity
    getRecentActivity: (params?: { limit?: number; type?: string; repo?: string }) =>
        axios.get(`${API_BASE_URL}/activity`, { params }),
};

export default analyticsService; 