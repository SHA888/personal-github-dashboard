import React, { useState, useEffect } from "react";
import { Box, Typography, LinearProgress, Grid, CircularProgress } from "@mui/material";
import RepositoryActivity from "./RepositoryActivity";
import ActivityTrends from "./ActivityTrends";
import AnalyticsLayout from "./AnalyticsLayout";
import { apiService } from "../../services/api";
import { useQuery } from "react-query";

interface Filters {
  timeRange: string;
  repo: string;
}

const AnalyticsDashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters] = useState<Filters>({
    timeRange: "7d",
    repo: "all",
  });

  const { data: activityResponse, isLoading: isActivityLoading } = useQuery(
    ['activity'],
    () => apiService.getRepositoryActivity(filters.owner, filters.repo)
  );

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const [activityResponse] = await Promise.all([
          apiService.getRepositoryActivity(filters.owner, filters.repo),
        ]);
        setError(null);
      } catch (err) {
        console.error("Error fetching data:", err);
        setError("Failed to fetch analytics data");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [filters.owner, filters.repo]);

  if (isActivityLoading) {
    return <CircularProgress />;
  }

  if (loading) {
    return (
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        minHeight="100vh"
      >
        <LinearProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        minHeight="100vh"
      >
        <Typography color="error">{error}</Typography>
      </Box>
    );
  }

  const activityData = activityResponse?.data || [];

  return (
    <AnalyticsLayout>
      <Box sx={{ flexGrow: 1, p: 3 }}>
        <Typography variant="h4" gutterBottom>
          Repository Analytics
        </Typography>
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <RepositoryActivity filters={filters} />
          </Grid>
          <Grid item xs={12}>
            <ActivityTrends filters={filters} />
          </Grid>
        </Grid>
      </Box>
    </AnalyticsLayout>
  );
};

export default AnalyticsDashboard;
