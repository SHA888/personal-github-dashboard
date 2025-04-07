import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api';

export interface Repository {
    id: number;
    name: string;
    owner: string;
    url: string;
}

export interface ActivityData {
    commit_activity: {
        daily: number[];
        weekly: number[];
        monthly: number[];
    };
    issue_metrics: {
        open: number;
        closed: number;
        average_resolution_time: string;
    };
    pr_metrics: {
        open: number;
        merged: number;
        average_merge_time: string;
    };
}

class ApiService {
    async listRepositories(): Promise<{ data: { repositories: Repository[] } }> {
        const response = await axios.get(`${API_BASE_URL}/repos`);
        return response;
    }

    async getRepositoryAnalytics(owner: string, repo: string): Promise<{ data: ActivityData }> {
        const response = await axios.get(`${API_BASE_URL}/analytics/repos/${owner}/${repo}`);
        return response;
    }

    async getActivityTrends(owner: string, repo: string, timeRange: string): Promise<{ data: ActivityData }> {
        const response = await axios.get(`${API_BASE_URL}/analytics/activity/${owner}/${repo}`, {
            params: { timeRange }
        });
        return response;
    }
}

export const apiService = new ApiService(); 