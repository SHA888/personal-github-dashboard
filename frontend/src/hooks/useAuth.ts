import { useState, useEffect } from 'react';
import { useMutation } from 'react-query';
import { apiService } from '../services/api';
import { User } from '../types/user';

export const useAuth = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isUserLoading, setLoading] = useState(true);
  const [isUserError, setError] = useState(false);

  // Parse JWT from localStorage
  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      try {
        const payload = JSON.parse(atob(token.split('.')[1]));
        setUser({ login: payload.sub } as User);
      } catch {
        setError(true);
      }
    }
    setLoading(false);
  }, []);

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
      // Clear auth token and user state
      localStorage.removeItem('auth_token');
      setUser(null);
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
