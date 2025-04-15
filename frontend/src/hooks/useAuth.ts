import { useState, useEffect } from 'react';
import { User } from '../types/user';
import { useQuery } from 'react-query';

interface AuthState {
    isAuthenticated: boolean;
    user: User | null;
    isLoading: boolean;
    error: string | null;
}

export const useAuth = () => {
    const [state, setState] = useState<AuthState>({
        isAuthenticated: false,
        user: null,
        isLoading: true,
        error: null,
    });

    const checkAuth = async () => {
        try {
            const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/me`, {
                credentials: 'include', // Important for sending cookies
            });

            if (response.ok) {
                const user = await response.json();
                setState({
                    isAuthenticated: true,
                    user,
                    isLoading: false,
                    error: null,
                });
            } else {
                setState({
                    isAuthenticated: false,
                    user: null,
                    isLoading: false,
                    error: null,
                });
            }
        } catch (error) {
            setState({
                isAuthenticated: false,
                user: null,
                isLoading: false,
                error: 'Failed to check authentication status',
            });
        }
    };

    const logout = async () => {
        try {
            const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/logout`, {
                method: 'POST',
                credentials: 'include',
            });

            if (response.ok) {
                setState({
                    isAuthenticated: false,
                    user: null,
                    isLoading: false,
                    error: null,
                });
            } else {
                throw new Error('Logout failed');
            }
        } catch (error) {
            setState(prev => ({
                ...prev,
                error: 'Failed to logout',
            }));
            throw error;
        }
    };

    useEffect(() => {
        checkAuth();
    }, []);

    const { data: user, isLoading: queryLoading } = useQuery(['user'], () => getCurrentUser(), {
        retry: false,
        onError: () => {
            // Handle error silently - user is not authenticated
        }
    });

    return {
        ...state,
        logout,
        checkAuth,
    };
};
