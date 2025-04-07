import React, { useState, useEffect } from "react";
import { Outlet } from "react-router-dom";
import { repositoryService } from "../../services/repositoryService";

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
        const response = await repositoryService.getRepositories();
        setRepositories(response.data || []);
        setError(null);
      } catch (error) {
        console.error("Error fetching repositories:", error);
        setError("Failed to load repositories");
        setRepositories([]);
      } finally {
        setLoading(false);
      }
    };

    fetchRepositories();
  }, []);

  const handleFilterChange = (key: keyof Filters, value: string) => {
    setFilters((prev) => ({ ...prev, [key]: value }));
  };

  const uniqueOwners = Array.from(new Set(repositories.map((r) => r.owner)));
  const filteredRepos = repositories.filter(
    (r) => !filters.owner || r.owner === filters.owner
  );

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-danger">{error}</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-white rounded-lg shadow-md p-6 mb-8">
          <h1 className="text-2xl font-bold text-gray-900 mb-6">
            GitHub Analytics Dashboard
          </h1>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div>
              <label
                htmlFor="timeRange"
                className="block text-sm font-medium text-gray-700 mb-2"
              >
                Time Range
              </label>
              <select
                id="timeRange"
                value={filters.timeRange}
                onChange={(e) => handleFilterChange("timeRange", e.target.value)}
                className="w-full rounded-md border-gray-300 shadow-sm focus:border-primary focus:ring-primary"
              >
                <option value="7d">Last 7 days</option>
                <option value="30d">Last 30 days</option>
                <option value="90d">Last 90 days</option>
              </select>
            </div>

            <div>
              <label
                htmlFor="owner"
                className="block text-sm font-medium text-gray-700 mb-2"
              >
                Owner
              </label>
              <select
                id="owner"
                value={filters.owner}
                onChange={(e) => handleFilterChange("owner", e.target.value)}
                className="w-full rounded-md border-gray-300 shadow-sm focus:border-primary focus:ring-primary"
              >
                <option value="">Select Owner</option>
                {uniqueOwners.map((owner) => (
                  <option key={owner} value={owner}>
                    {owner}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label
                htmlFor="repo"
                className="block text-sm font-medium text-gray-700 mb-2"
              >
                Repository
              </label>
              <select
                id="repo"
                value={filters.repo}
                onChange={(e) => handleFilterChange("repo", e.target.value)}
                className="w-full rounded-md border-gray-300 shadow-sm focus:border-primary focus:ring-primary"
                disabled={!filters.owner}
              >
                <option value="">Select Repository</option>
                {filteredRepos.map((repo) => (
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
    </div>
  );
};

export default Layout;
