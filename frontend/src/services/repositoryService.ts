import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api';

export interface Repository {
    id: number;
    name: string;
    owner: string;
    description: string | null;
    created_at: string;
    updated_at: string;
}

class RepositoryService {
    async getRepositories(): Promise<{ data: Repository[] }> {
        const response = await axios.get(`${API_BASE_URL}/repos`);
        return response;
    }
}

export const repositoryService = new RepositoryService(); 