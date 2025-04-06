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
        const response = await apiService.getRepositoryActivity(
          filters.owner,
          filters.repo,
        );
        const transformedData = {
          dates: response.data.dates,
          commit_counts: response.data.commits,
        };
        setTrendData(transformedData);
        setError(null);
      } catch (error) {
        console.error("Error fetching trend data:", error);
        setError("Failed to load trend data");
      } finally {
        setLoading(false);
      }
    };

    fetchTrendData();
  }, [filters.timeRange, filters.owner, filters.repo]);

  if (loading) {
    return <LinearProgress />;
  }

  if (error) {
    return <Typography color="error">{error}</Typography>;
  }

  if (!trendData) {
    return <Typography>No trend data available</Typography>;
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
    <Box>
      <Typography variant="h4" gutterBottom>
        Commit Trends
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Commit Activity
              </Typography>
              <Box height={300}>
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
              </Box>
            </CardContent>
          </StatCard>
        </Grid>
      </Grid>
    </Box>
  );
};

export default Trends;
