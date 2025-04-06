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

const StatCard = styled(Card)(({ theme }) => ({
  height: "100%",
  display: "flex",
  flexDirection: "column",
}));

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryActivityProps {
  filters: Filters;
}

interface ActivityData {
  dates: string[];
  total_activity: number[];
  commits: number[];
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
  }, [filters.timeRange, filters.owner, filters.repo]);

  if (loading) {
    return <LinearProgress />;
  }

  if (error) {
    return <Typography color="error">{error}</Typography>;
  }

  if (!activityData) {
    return <Typography>No activity data available</Typography>;
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
    <Box>
      <Typography variant="h4" gutterBottom>
        Repository Activity
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Daily Activity
              </Typography>
              <Box height={300}>
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
              </Box>
            </CardContent>
          </StatCard>
        </Grid>
      </Grid>
    </Box>
  );
};

export default RepositoryActivity;
