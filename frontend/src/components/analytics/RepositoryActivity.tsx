import React, { useState, useEffect } from "react";
import { Bar } from "react-chartjs-2";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { apiService, ActivityData } from "../../services/api";

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
);

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryActivityProps {
  filters: Filters;
}

const RepositoryActivity: React.FC<RepositoryActivityProps> = ({ filters }) => {
  const [activityData, setActivityData] = useState<ActivityData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchActivityData = async () => {
      try {
        setLoading(true);
        const response = await apiService.getRepositoryActivity(
          filters.owner,
          filters.repo,
          30, // Default to 30 days
        );
        setActivityData(response.data);
        setError(null);
      } catch (error) {
        console.error("Error fetching repository activity:", error);
        setError("Failed to load repository activity data");
      } finally {
        setLoading(false);
      }
    };

    fetchActivityData();
  }, [filters.owner, filters.repo]);

  if (loading) {
    return <div className="w-full h-1 bg-gray-200 animate-pulse" />;
  }

  if (error) {
    return <p className="text-danger">{error}</p>;
  }

  if (!activityData) {
    return <p className="text-secondary">No activity data available</p>;
  }

  const dailyActivityChartData = {
    labels: activityData.dates.map((date: string) =>
      new Date(date).toLocaleDateString(),
    ),
    datasets: [
      {
        label: "Total Activity",
        data: activityData.total_activity,
        backgroundColor: "rgba(75, 192, 192, 0.6)",
      },
      {
        label: "Commits",
        data: activityData.commits,
        backgroundColor: "rgba(255, 99, 132, 0.6)",
      },
    ],
  };

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold text-gray-900">Repository Activity</h2>

      <div className="grid grid-cols-1 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            Daily Activity
          </h3>
          <div className="h-[300px]">
            <Bar
              data={dailyActivityChartData}
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
  );
};

export default RepositoryActivity;
