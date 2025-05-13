import { createAsyncThunk } from '@reduxjs/toolkit';
import apiClient, { patAuth, logout as apiLogout } from '../services/api';
import { User } from './authSlice';

export const fetchCurrentUser = createAsyncThunk<User>(
  'auth/fetchCurrentUser',
  async (_, { rejectWithValue }) => {
    try {
      const res = await apiClient.get<User>('/api/user');
      return res.data;
    } catch (err: unknown) {
      return rejectWithValue(
        (err as { response?: { data?: { error?: string } } }).response?.data
          ?.error ||
          (err as Error).message ||
          'Failed to fetch user',
      );
    }
  },
);

export const loginWithPatThunk = createAsyncThunk<
  { user: User; token: string },
  string,
  { rejectValue: string }
>('auth/loginWithPat', async (pat, { rejectWithValue }) => {
  try {
    const resp = await patAuth(pat);
    localStorage.setItem('auth_token', resp.jwt);
    const userResp = await apiClient.get<User>('/api/user');
    return { user: userResp.data, token: resp.jwt };
  } catch (err: unknown) {
    return rejectWithValue(
      (err as { response?: { data?: { error?: string } } }).response?.data
        ?.error ||
        (err as Error).message ||
        'Login failed',
    );
  }
});

export const logoutThunk = createAsyncThunk<void>(
  'auth/logout',
  async (_, { rejectWithValue }) => {
    try {
      await apiLogout();
      localStorage.removeItem('auth_token');
    } catch (err: unknown) {
      return rejectWithValue(
        (err as { response?: { data?: { error?: string } } }).response?.data
          ?.error ||
          (err as Error).message ||
          'Logout failed',
      );
    }
  },
);
