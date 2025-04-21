import { useQuery } from '@tanstack/react-query';
import apiClient from '../services/api';

export interface RepositoryInfo {
  name: string;
  description: string | null;
  html_url: string;
  stargazers_count: number;
  forks_count: number;
  open_issues_count: number;
}

export const useRepositories = (page: number = 1, perPage: number = 10) => {
  return useQuery<RepositoryInfo[], Error>({
    queryKey: ['repositories', page],
    queryFn: async () => {
      const response = await apiClient.get<RepositoryInfo[]>(
        `/api/repositories?page=${page}&per_page=${perPage}`,
      );
      return response.data;
    },
    staleTime: 1000 * 60, // 1 minute
  });
};
