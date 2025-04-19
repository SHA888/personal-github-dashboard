import { useNavigate } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import apiClient, {
  patAuth,
  logout as apiLogout,
  PatResponse,
} from '../services/api';

interface User {
  login: string;
}

export const useAuth = () => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  // Fetch current user
  const { data: user, isLoading: loading } = useQuery<User, Error>({
    queryKey: ['currentUser'],
    queryFn: async () => {
      const res = await apiClient.get<User>('/api/user');
      return res.data;
    },
    retry: false,
  });

  // Login with PAT mutation
  const loginMutation = useMutation<PatResponse, Error, string>({
    mutationFn: patAuth,
    onSuccess: (data) => {
      localStorage.setItem('auth_token', data.jwt);
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
      navigate('/dashboard', { replace: true });
    },
  });

  // Logout mutation
  const logoutMutation = useMutation<void, Error>({
    mutationFn: apiLogout,
    onSuccess: () => {
      localStorage.removeItem('auth_token');
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
      navigate('/login', { replace: true });
    },
  });

  const loginWithGitHub = () => {
    window.location.href = `${import.meta.env.VITE_API_BASE_URL}/auth/login`;
  };

  const loginWithPat = (pat: string) => loginMutation.mutate(pat);
  const logout = () => logoutMutation.mutate();
  const isAuthenticated = !!user;

  return {
    user,
    loading,
    loginWithGitHub,
    loginWithPat,
    logout,
    isAuthenticated,
    loginMutation,
    logoutMutation,
  };
};
