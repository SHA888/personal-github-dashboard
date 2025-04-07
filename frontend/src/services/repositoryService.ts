import axios from "axios";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:8000";

export interface Repository {
    id: number;
    name: string;
    owner: string;
}

export const repositoryService = {
    getRepositories: async () => {
        const response = await axios.get<{ data: Repository[] }>(
            `${API_BASE_URL}/api/repositories`
        );
        return response.data;
    },
}; 