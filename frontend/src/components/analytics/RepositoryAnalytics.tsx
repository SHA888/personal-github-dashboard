import React, { useState, useEffect } from "react";
import {
  Box,
  Typography,
  LinearProgress,
  Grid,
  Card,
  CardContent,
  styled,
} from "@mui/material";
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

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
);

const StatCard = styled(Card)({
  height: "100%",
  display: "flex",
  flexDirection: "column",
});

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryActivityProps {
  filters: Filters;
}

interface ActivityData {
  commit_activity: {
    daily: number[];
    weekly: number[];
    monthly: number[];
  };
  issue_metrics: {
    open: number;
    closed: number;
    average_resolution_time: string;
  };
  pr_metrics: {
    open: number;
    merged: number;
    average_merge_time: string;
  };
}

const RepositoryActivity: React.FC<RepositoryActivityProps> = ({ filters }) => {
  const [activityData, setActivityData] = useState<ActivityData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchActivityData = async () => {
      try {
        setLoading(true);
        const response = await apiService.getRepositoryAnalytics(
          filters.owner,
          filters.repo,
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
    return <LinearProgress />;
  }

  if (error) {
    return <Typography color="error">{error}</Typography>;
  }

  if (!activityData) {
    return <Typography>No activity data available</Typography>;
  }

  const commitActivityChartData = {
    labels: activityData.commit_activity.daily.map(
      (_, index) => `Day ${index + 1}`,
    ),
    datasets: [
      {
        label: "Daily Commits",
        data: activityData.commit_activity.daily,
        backgroundColor: "rgba(75, 192, 192, 0.6)",
      },
    ],
  };

  const issueMetricsChartData = {
    labels: ["Open", "Closed"],
    datasets: [
      {
        label: "Issues",
        data: [
          activityData.issue_metrics.open,
          activityData.issue_metrics.closed,
        ],
        backgroundColor: ["rgba(255, 99, 132, 0.6)", "rgba(54, 162, 235, 0.6)"],
      },
    ],
  };

  const prMetricsChartData = {
    labels: ["Open", "Merged"],
    datasets: [
      {
        label: "Pull Requests",
        data: [activityData.pr_metrics.open, activityData.pr_metrics.merged],
        backgroundColor: ["rgba(255, 206, 86, 0.6)", "rgba(75, 192, 192, 0.6)"],
      },
    ],
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Repository Activity
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Commit Activity
              </Typography>
              <Bar
                data={commitActivityChartData}
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
            </CardContent>
          </StatCard>
        </Grid>

        <Grid item xs={12} md={6}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Issue Metrics
              </Typography>
              <Bar
                data={issueMetricsChartData}
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
              <Typography variant="body2" sx={{ mt: 2 }}>
                Average Resolution Time:{" "}
                {activityData.issue_metrics.average_resolution_time}
              </Typography>
            </CardContent>
          </StatCard>
        </Grid>

        <Grid item xs={12} md={6}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Pull Request Metrics
              </Typography>
              <Bar
                data={prMetricsChartData}
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
              <Typography variant="body2" sx={{ mt: 2 }}>
                Average Merge Time: {activityData.pr_metrics.average_merge_time}
              </Typography>
            </CardContent>
          </StatCard>
        </Grid>
      </Grid>
    </Box>
  );
};

export default RepositoryActivity;
