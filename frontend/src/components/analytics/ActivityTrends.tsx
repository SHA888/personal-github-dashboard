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
import { analyticsService } from "../../services/analyticsService";

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

interface ActivityTrendsProps {
  filters: Filters;
}

interface TrendData {
  commit_activity: {
    daily: number[];
    weekly: number[];
    monthly: number[];
  };
}

const ActivityTrends: React.FC<ActivityTrendsProps> = ({ filters }) => {
  const [trendData, setTrendData] = useState<TrendData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchTrendData = async () => {
      try {
        setLoading(true);
        const response = await analyticsService.getRepositoryAnalytics(
          filters.owner,
          filters.repo,
          filters.timeRange
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
  }, [filters]);

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-8">
        <p className="text-red-600">{error}</p>
      </div>
    );
  }

  if (!trendData) {
    return (
      <div className="text-center py-8">
        <p className="text-gray-600">No trend data available</p>
      </div>
    );
  }

  const dailyCommitChartData = {
    labels: trendData.commit_activity.daily.map((_, index) => `Day ${index + 1}`),
    datasets: [
      {
        label: "Daily Commits",
        data: trendData.commit_activity.daily,
        borderColor: "rgb(75, 192, 192)",
        tension: 0.1,
      },
    ],
  };

  const weeklyCommitChartData = {
    labels: trendData.commit_activity.weekly.map((_, index) => `Week ${index + 1}`),
    datasets: [
      {
        label: "Weekly Commits",
        data: trendData.commit_activity.weekly,
        borderColor: "rgb(255, 99, 132)",
        tension: 0.1,
      },
    ],
  };

  const monthlyCommitChartData = {
    labels: trendData.commit_activity.monthly.map((_, index) => `Month ${index + 1}`),
    datasets: [
      {
        label: "Monthly Commits",
        data: trendData.commit_activity.monthly,
        borderColor: "rgb(54, 162, 235)",
        tension: 0.1,
      },
    ],
  };

  return (
    <div className="space-y-8">
      <h2 className="text-2xl font-bold text-gray-900">Commit Trends</h2>

      <div className="grid grid-cols-1 gap-8">
        <div className="card">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Daily Commit Trends</h3>
          <div className="h-64">
            <Line
              data={dailyCommitChartData}
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
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Weekly Commit Trends</h3>
            <div className="h-64">
              <Line
                data={weeklyCommitChartData}
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
            </div>
          </div>

          <div className="card">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Monthly Commit Trends</h3>
            <div className="h-64">
              <Line
                data={monthlyCommitChartData}
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
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ActivityTrends;
