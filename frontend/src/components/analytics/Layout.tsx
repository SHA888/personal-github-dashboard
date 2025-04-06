import React, { useState, useEffect } from "react";
import { Outlet } from "react-router-dom";
import { analyticsService } from "../../services/analyticsService";

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface Repository {
  id: number;
  name: string;
  owner: string;
}

const Layout: React.FC = () => {
  const [filters, setFilters] = useState<Filters>({
    timeRange: "30d",
    owner: "",
    repo: "",
  });
  const [repositories, setRepositories] = useState<Repository[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchRepositories = async () => {
      try {
        setLoading(true);
        const response = await analyticsService.getRepositories();
        setRepositories(response.data);
        setError(null);
      } catch (error) {
        console.error("Error fetching repositories:", error);
        setError("Failed to load repositories");
      } finally {
        setLoading(false);
      }
    };

    fetchRepositories();
  }, []);

  const handleFilterChange = (key: keyof Filters, value: string) => {
    setFilters((prev) => ({ ...prev, [key]: value }));
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-screen">
        <div className="loading-spinner" />
      </div>
    );
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="section-title">GitHub Analytics</h1>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Time Range
            </label>
            <select
              className="input"
              value={filters.timeRange}
              onChange={(e) => handleFilterChange("timeRange", e.target.value)}
            >
              <option value="7d">Last 7 days</option>
              <option value="30d">Last 30 days</option>
              <option value="90d">Last 90 days</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Owner
            </label>
            <input
              type="text"
              className="input"
              value={filters.owner}
              onChange={(e) => handleFilterChange("owner", e.target.value)}
              placeholder="Enter owner"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Repository
            </label>
            <select
              className="input"
              value={filters.repo}
              onChange={(e) => handleFilterChange("repo", e.target.value)}
              disabled={!filters.owner}
            >
              <option value="">Select repository</option>
              {repositories
                .filter((repo) => repo.owner === filters.owner)
                .map((repo) => (
                  <option key={repo.id} value={repo.name}>
                    {repo.name}
                  </option>
                ))}
            </select>
          </div>
        </div>
      </div>

      <Outlet context={filters} />
    </div>
  );
};

export default Layout;
