import React, { useState, useEffect } from "react";
import { useOutletContext } from "react-router-dom";
import { analyticsService } from "../../services/analyticsService";
import { Line } from "react-chartjs-2";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryAnalytics {
  dates: string[];
  total_activity: number[];
  commits: number[];
}

const RepositoryAnalytics: React.FC = () => {
  const filters = useOutletContext<Filters>();
  const [analytics, setAnalytics] = useState<RepositoryAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchAnalytics = async () => {
      if (!filters.owner || !filters.repo) {
        setLoading(false);
        return;
      }

      try {
        setLoading(true);
        const response = await analyticsService.getRepositoryAnalytics(
          filters.owner,
          filters.repo,
          filters.timeRange
        );
        setAnalytics(response.data || null);
        setError(null);
      } catch (error) {
        console.error("Error fetching analytics:", error);
        setError("Failed to load analytics data");
        setAnalytics(null);
      } finally {
        setLoading(false);
      }
    };

    fetchAnalytics();
  }, [filters]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-danger">{error}</div>
      </div>
    );
  }

  if (!analytics || !analytics.dates.length) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">No analytics data available</div>
      </div>
    );
  }

  const chartData = {
    labels: analytics.dates,
    datasets: [
      {
        label: "Total Activity",
        data: analytics.total_activity,
        borderColor: "rgb(34, 197, 94)",
        backgroundColor: "rgba(34, 197, 94, 0.5)",
        tension: 0.1,
      },
      {
        label: "Commits",
        data: analytics.commits,
        borderColor: "rgb(59, 130, 246)",
        backgroundColor: "rgba(59, 130, 246, 0.5)",
        tension: 0.1,
      },
    ],
  };

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: "top" as const,
      },
      title: {
        display: true,
        text: "Repository Activity",
      },
    },
    scales: {
      y: {
        beginAtZero: true,
      },
    },
  };

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-lg font-semibold text-gray-800 mb-4">
        Activity Overview
      </h2>
      <div className="h-96">
        <Line data={chartData} options={chartOptions} />
      </div>
    </div>
  );
};

export default RepositoryAnalytics;
