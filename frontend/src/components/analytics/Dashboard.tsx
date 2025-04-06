import React, { useState, useEffect } from "react";
import { useOutletContext } from "react-router-dom";
import RepositoryAnalytics from "./RepositoryAnalytics";
import ActivityTrends from "./ActivityTrends";
import { analyticsService } from "../../services/analyticsService";

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

const Dashboard: React.FC = () => {
  const filters = useOutletContext<Filters>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      if (!filters.owner || !filters.repo) return;

      try {
        setLoading(true);
        await analyticsService.getRepositoryAnalytics(
          filters.owner,
          filters.repo,
          filters.timeRange
        );
        setError(null);
      } catch (error) {
        console.error("Error fetching analytics data:", error);
        setError("Failed to load analytics data");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [filters]);

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="loading-spinner" />
      </div>
    );
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  if (!filters.owner || !filters.repo) {
    return (
      <div className="empty-state">
        Please select an owner and repository to view analytics
      </div>
    );
  }

  return (
    <div className="space-y-8">
      <RepositoryAnalytics filters={filters} />
      <ActivityTrends filters={filters} />
    </div>
  );
};

export default Dashboard;
