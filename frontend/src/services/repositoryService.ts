import axios from "axios";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8000/api";

export interface Repository {
  id: number;
  owner: string;
  name: string;
  description: string | null;
  language: string | null;
  stars: number;
  forks: number;
  open_issues: number;
  is_private: boolean;
  created_at: string;
  updated_at: string;
}

export interface CommitActivity {
  id: number;
  sha: string;
  message: string;
  author_name: string;
  author_email: string;
  created_at: string;
}

class RepositoryService {
  async getRepositories(): Promise<{ data: Repository[] }> {
    const response = await axios.get(`${API_BASE_URL}/repositories`);
    return response;
  }

  async getRepository(
    owner: string,
    name: string,
  ): Promise<{ data: Repository }> {
    const response = await axios.get(
      `${API_BASE_URL}/repositories/${owner}/${name}`,
    );
    return response;
  }

  async getRepositoryActivity(
    owner: string,
    name: string,
  ): Promise<{ data: CommitActivity[] }> {
    const response = await axios.get(
      `${API_BASE_URL}/repositories/${owner}/${name}/activity`,
    );
    return response;
  }

  async addRepository(owner: string, name: string): Promise<void> {
    await axios.post(`${API_BASE_URL}/repositories`, { owner, name });
  }
}

export const repositoryService = new RepositoryService();
