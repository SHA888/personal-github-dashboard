import { useQuery, useMutation, useQueryClient } from 'react-query';
import { apiService } from '../services/api';
import { AxiosError } from 'axios';
import { User } from '../types/user';

export const useAuth = () => {
  const queryClient = useQueryClient();

  // Fetch current user data
  const {
    data: user,
    isLoading: isUserLoading,
    // error, // Commented out unused variable
    isError: isUserError,
  } = useQuery<User, AxiosError>('currentUser', apiService.getCurrentUser, {
    retry: false, // Don't retry on initial fetch; handled by redirects
    refetchOnWindowFocus: false, // Avoid unnecessary refetches
    staleTime: Infinity, // User data rarely changes without action
    onError: (err) => {
      if (err.response?.status === 401) {
        // Handle unauthorized globally if needed, or let components handle
        console.log('User is not authenticated.');
      }
    },
  });

  // GitHub Login Mutation (redirects)
  const githubLoginMutation = useMutation(() => apiService.githubLogin(), {
    onSuccess: (data) => {
      // Redirect the user to the GitHub authorization URL
      if (data.redirectUrl) {
        window.location.href = data.redirectUrl;
      }
    },
    onError: (/* error */) => {
      // Handle login initiation error (e.g., show a notification)
      console.error('GitHub login initiation failed');
    },
  });

  // Logout Mutation
  const logoutMutation = useMutation(() => apiService.logout(), {
    onSuccess: () => {
      // Clear user data from cache and redirect
      queryClient.setQueryData('currentUser', null);
      queryClient.removeQueries('currentUser');
      // Optionally redirect to login page or home page
      window.location.href = '/'; // Redirect to home after logout
    },
    onError: (/* error */) => {
      // Handle logout error
      console.error('Logout failed');
    },
  });

  // Derived state for authentication status
  const isAuthenticated = !!user && !isUserError;

  return {
    user: isAuthenticated ? user : null,
    isLoading: isUserLoading,
    isAuthenticated,
    githubLogin: githubLoginMutation.mutate,
    isLoggingIn: githubLoginMutation.isLoading,
    logout: logoutMutation.mutate,
    isLoggingOut: logoutMutation.isLoading,
  };
};

// Placeholder hook for fetching user data from backend
export const useUser = () => {
  // const { data: user, isLoading: queryLoading } = useQuery('user', apiService.getCurrentUser); // Commented out

  return {
    user: null, // Replace with actual user data
    // isLoading: queryLoading,
    isLoading: false, // Placeholder
  };
};
