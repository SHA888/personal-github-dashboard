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
import { apiService } from "../../services/api";
import { Box, CircularProgress, Typography, Link } from "@mui/material";

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

interface ActivityData {
  date: string;
  commits: number;
  additions: number;
  deletions: number;
}

interface RepositoryActivityProps {
  filters: Filters;
}

const RepositoryActivity: React.FC<RepositoryActivityProps> = ({ filters }) => {
  const [activityData, setActivityData] = useState<ActivityData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isEmptyRepo, setIsEmptyRepo] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);
        setIsEmptyRepo(false);

        const response = await apiService.getRepositoryActivity(
          filters.owner,
          filters.repo,
        );
        setActivityData(response.data as ActivityData[]);
      } catch (err: any) {
        console.error("Error fetching activity data:", err);
        setError(err.message);
        setIsEmptyRepo(err.isEmptyRepo || false);
        // Keep old data if there was any
        setActivityData((prev) => prev);
      } finally {
        setLoading(false);
      }
    };

    if (filters.owner && filters.repo) {
      fetchData();
    }
  }, [filters.owner, filters.repo]);

  if (loading && !activityData) {
    return (
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        minHeight="200px"
      >
        <CircularProgress />
      </Box>
    );
  }

  if (isEmptyRepo) {
    return (
      <Box p={3} bgcolor="background.paper" borderRadius={1}>
        <Typography variant="body1" color="textSecondary" align="center">
          This repository is empty. Analytics will be available once commits are
          added.
        </Typography>
        <Typography variant="body2" color="textSecondary" align="center" mt={1}>
          <Link
            href={`https://github.com/${filters.owner}/${filters.repo}`}
            target="_blank"
            rel="noopener noreferrer"
          >
            View repository on GitHub
          </Link>
        </Typography>
      </Box>
    );
  }

  if (error && !activityData) {
    return (
      <Box p={3} bgcolor="error.light" borderRadius={1}>
        <Typography color="error">{error}</Typography>
      </Box>
    );
  }

  const dailyActivityChartData = {
    labels: activityData.map((data: ActivityData) => data.date),
    datasets: [
      {
        label: "Total Activity",
        data: activityData.map((data: ActivityData) => data.commits),
        backgroundColor: "rgba(75, 192, 192, 0.6)",
      },
      {
        label: "Commits",
        data: activityData.map((data: ActivityData) => data.commits),
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
