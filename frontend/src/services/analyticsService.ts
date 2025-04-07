import axios from "axios";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:8000";

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
    getRepositoryAnalytics: async (owner: string, repo: string, timeRange: string) => {
        const response = await axios.get<AnalyticsResponse>(
            `${API_BASE_URL}/analytics/repos/${owner}/${repo}?timeRange=${timeRange}`
        );
        return response.data;
    },

    getUserAnalytics: async (username: string) => {
        const response = await axios.get<AnalyticsResponse>(
            `${API_BASE_URL}/analytics/users/${username}`
        );
        return response.data;
    },

    getActivityTrends: async (owner: string, repo: string, timeRange: string) => {
        const response = await axios.get<AnalyticsResponse>(
            `${API_BASE_URL}/analytics/trends/${owner}/${repo}?timeRange=${timeRange}`
        );
        return response.data;
    },

    // Activity
    getRecentActivity: (params?: { limit?: number; type?: string; repo?: string }) =>
        axios.get(`${API_BASE_URL}/activity`, { params }),
}; 