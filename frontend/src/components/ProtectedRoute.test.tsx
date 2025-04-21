import { vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MemoryRouter, Routes, Route } from 'react-router-dom';
import ProtectedRoute from './ProtectedRoute';
import * as authHook from '../hooks/useAuth';

describe('ProtectedRoute', () => {
  it('renders outlet when authenticated', () => {
    vi.spyOn(authHook, 'useAuth').mockReturnValue({
      user: { login: 'testuser' },
      loading: false,
      loginWithGitHub: vi.fn(),
      loginWithPat: vi.fn(),
      logout: vi.fn(),
      isAuthenticated: true,
      loginMutation: { mutate: vi.fn() },
      logoutMutation: { mutate: vi.fn() },
    });
    render(
      <MemoryRouter initialEntries={['/dashboard']}>
        <Routes>
          <Route element={<ProtectedRoute />}>
            <Route path="/dashboard" element={<div>Dashboard</div>} />
          </Route>
        </Routes>
      </MemoryRouter>,
    );
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
  });

  it('redirects to login when not authenticated', () => {
    vi.spyOn(authHook, 'useAuth').mockReturnValue({
      user: undefined,
      loading: false,
      loginWithGitHub: vi.fn(),
      loginWithPat: vi.fn(),
      logout: vi.fn(),
      isAuthenticated: false,
      loginMutation: { mutate: vi.fn() },
      logoutMutation: { mutate: vi.fn() },
    });
    render(
      <MemoryRouter initialEntries={['/dashboard']}>
        <Routes>
          <Route element={<ProtectedRoute />}>
            <Route path="/dashboard" element={<div>Dashboard</div>} />
          </Route>
          <Route path="/login" element={<div>Login</div>} />
        </Routes>
      </MemoryRouter>,
    );
    expect(screen.getByText('Login')).toBeInTheDocument();
  });
});
