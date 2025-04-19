import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { patAuth, logout as apiLogout } from "../services/api";

interface User {
  login: string;
}

export const useAuth = () => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    const token = localStorage.getItem("auth_token");
    if (token) {
      try {
        const payload = JSON.parse(atob(token.split(".")[1]));
        setUser({ login: payload.sub });
      } catch {
        setUser(null);
      }
    }
    setLoading(false);
  }, []);

  const loginWithGitHub = () => {
    window.location.href = `${import.meta.env.VITE_API_BASE_URL}/auth/login`;
  };

  const loginWithPat = async (pat: string) => {
    const { jwt } = await patAuth(pat);
    localStorage.setItem("auth_token", jwt);
    window.location.href = "/login";
  };

  const logout = async () => {
    await apiLogout();
    localStorage.removeItem("auth_token");
    setUser(null);
    navigate("/login");
  };

  return {
    user,
    loading,
    loginWithGitHub,
    loginWithPat,
    logout,
    isAuthenticated: !!user,
  };
};
