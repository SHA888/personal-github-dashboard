import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Card, CardContent, styled, Grid } from '@mui/material';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { apiService } from '../../services/api';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

const StatCard = styled(Card)({
  height: '100%',
  display: 'flex',
  flexDirection: 'column',
});

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface TrendsProps {
  filters: Filters;
}

interface TrendData {
  commit_activity: {
    daily: number[];
    weekly: number[];
    monthly: number[];
  };
}

const Trends: React.FC<TrendsProps> = ({ filters }) => {
  const [trendData, setTrendData] = useState<TrendData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchTrendData = async () => {
      try {
        setLoading(true);
        const response = await apiService.getRepositoryAnalytics(filters.owner, filters.repo);
        setTrendData(response.data);
        setError(null);
      } catch (error) {
        console.error('Error fetching trend data:', error);
        setError('Failed to load trend data');
      } finally {
        setLoading(false);
      }
    };

    fetchTrendData();
  }, [filters.owner, filters.repo]);

  if (loading) {
    return <LinearProgress />;
  }

  if (error) {
    return <Typography color="error">{error}</Typography>;
  }

  if (!trendData) {
    return <Typography>No trend data available</Typography>;
  }

  const dailyCommitChartData = {
    labels: trendData.commit_activity.daily.map((_, index) => `Day ${index + 1}`),
    datasets: [
      {
        label: 'Daily Commits',
        data: trendData.commit_activity.daily,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
      },
    ],
  };

  const weeklyCommitChartData = {
    labels: trendData.commit_activity.weekly.map((_, index) => `Week ${index + 1}`),
    datasets: [
      {
        label: 'Weekly Commits',
        data: trendData.commit_activity.weekly,
        borderColor: 'rgb(255, 99, 132)',
        tension: 0.1,
      },
    ],
  };

  const monthlyCommitChartData = {
    labels: trendData.commit_activity.monthly.map((_, index) => `Month ${index + 1}`),
    datasets: [
      {
        label: 'Monthly Commits',
        data: trendData.commit_activity.monthly,
        borderColor: 'rgb(54, 162, 235)',
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
                Daily Commit Trends
              </Typography>
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
            </CardContent>
          </StatCard>
        </Grid>

        <Grid item xs={12} md={6}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Weekly Commit Trends
              </Typography>
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
            </CardContent>
          </StatCard>
        </Grid>

        <Grid item xs={12} md={6}>
          <StatCard>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Monthly Commit Trends
              </Typography>
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
            </CardContent>
          </StatCard>
        </Grid>
      </Grid>
    </Box>
  );
};

export default Trends;
