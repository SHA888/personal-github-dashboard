import React, { useState, useEffect } from "react";
import { Pie } from "react-chartjs-2";
import {
  Chart as ChartJS,
  ArcElement,
  Tooltip,
  Legend,
} from "chart.js";
import { analyticsService } from "../../services/analyticsService";
import "../../styles/analytics.css";

ChartJS.register(ArcElement, Tooltip, Legend);

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryAnalyticsProps {
  filters: Filters;
}

interface AnalyticsData {
  commit_stats: {
    total: number;
    by_author: Record<string, number>;
  };
  pull_request_stats: {
    open: number;
    closed: number;
    merged: number;
  };
  issue_stats: {
    open: number;
    closed: number;
  };
}

const RepositoryAnalytics: React.FC<RepositoryAnalyticsProps> = ({ filters }) => {
  const [analyticsData, setAnalyticsData] = useState<AnalyticsData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchAnalyticsData = async () => {
      try {
        setLoading(true);
        const response = await analyticsService.getRepositoryAnalytics(
          filters.owner,
          filters.repo,
          filters.timeRange
        );
        setAnalyticsData(response.data);
        setError(null);
      } catch (error) {
        console.error("Error fetching analytics data:", error);
        setError("Failed to load analytics data");
      } finally {
        setLoading(false);
      }
    };

    fetchAnalyticsData();
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

  if (!analyticsData) {
    return <div className="empty-state">No analytics data available</div>;
  }

  const chartColors = {
    primary: "rgba(99, 102, 241, 0.6)",    // indigo-500
    secondary: "rgba(139, 92, 246, 0.6)",  // violet-500
    success: "rgba(16, 185, 129, 0.6)",    // emerald-500
    warning: "rgba(245, 158, 11, 0.6)",    // amber-500
    danger: "rgba(239, 68, 68, 0.6)",      // red-500
  };

  const commitByAuthorData = {
    labels: Object.keys(analyticsData.commit_stats.by_author),
    datasets: [
      {
        data: Object.values(analyticsData.commit_stats.by_author),
        backgroundColor: [
          chartColors.primary,
          chartColors.secondary,
          chartColors.success,
          chartColors.warning,
          chartColors.danger,
        ],
      },
    ],
  };

  const pullRequestData = {
    labels: ["Open", "Closed", "Merged"],
    datasets: [
      {
        data: [
          analyticsData.pull_request_stats.open,
          analyticsData.pull_request_stats.closed,
          analyticsData.pull_request_stats.merged,
        ],
        backgroundColor: [
          chartColors.danger,
          chartColors.secondary,
          chartColors.success,
        ],
      },
    ],
  };

  const issueData = {
    labels: ["Open", "Closed"],
    datasets: [
      {
        data: [
          analyticsData.issue_stats.open,
          analyticsData.issue_stats.closed,
        ],
        backgroundColor: [
          chartColors.danger,
          chartColors.success,
        ],
      },
    ],
  };

  return (
    <div className="repository-analytics">
      <h2 className="section-title">Repository Analytics</h2>

      <div className="dashboard-grid">
        <div className="dashboard-card">
          <h3 className="card-title">Total Commits</h3>
          <p className="stat-value">{analyticsData.commit_stats.total}</p>
        </div>

        <div className="dashboard-card">
          <h3 className="card-title">Commits by Author</h3>
          <div className="chart-container">
            <Pie
              data={commitByAuthorData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
              }}
            />
          </div>
        </div>

        <div className="dashboard-card">
          <h3 className="card-title">Pull Requests</h3>
          <div className="chart-container">
            <Pie
              data={pullRequestData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
              }}
            />
          </div>
        </div>

        <div className="dashboard-card">
          <h3 className="card-title">Issues</h3>
          <div className="chart-container">
            <Pie
              data={issueData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
              }}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default RepositoryAnalytics;
