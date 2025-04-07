import React, { useState, useEffect } from "react";
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
import { apiService } from "../../services/api";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
);

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface TrendsProps {
  filters: Filters;
}

interface TrendData {
  dates: string[];
  commit_counts: number[];
}

const Trends: React.FC<TrendsProps> = ({ filters }) => {
  const [trendData, setTrendData] = useState<TrendData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchTrendData = async () => {
      try {
        setLoading(true);
        const response = await apiService.getRepositoryTrends(
          filters.owner,
          filters.repo,
          30, // Default to 30 days
        );
        setTrendData(response.data);
        setError(null);
      } catch (error) {
        console.error("Error fetching trend data:", error);
        setError("Failed to load trend data");
      } finally {
        setLoading(false);
      }
    };

    fetchTrendData();
  }, [filters.owner, filters.repo]);

  if (loading) {
    return <div className="w-full h-1 bg-gray-200 animate-pulse" />;
  }

  if (error) {
    return <p className="text-danger">{error}</p>;
  }

  if (!trendData) {
    return <p className="text-secondary">No trend data available</p>;
  }

  const commitChartData = {
    labels: trendData.dates.map((date: string) =>
      new Date(date).toLocaleDateString(),
    ),
    datasets: [
      {
        label: "Daily Commits",
        data: trendData.commit_counts,
        borderColor: "rgb(75, 192, 192)",
        tension: 0.1,
      },
    ],
  };

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold text-gray-900">Commit Trends</h2>

      <div className="w-full p-4">
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            Commit Trends
          </h3>
          {loading ? (
            <div className="w-full h-1 bg-gray-200 animate-pulse" />
          ) : error ? (
            <p className="text-danger">{error}</p>
          ) : (
            <Line
              data={commitChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                  y: {
                    beginAtZero: true,
                  },
                },
              }}
            />
          )}
        </div>
      </div>
    </div>
  );
};

export default Trends;
