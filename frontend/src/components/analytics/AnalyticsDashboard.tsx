import { Box } from '@mui/material';
import { useEffect, useState } from 'react';
import { Grid, Paper, Typography, CircularProgress } from "@mui/material";
import { styled } from "@mui/material/styles";
import RepositoryActivity from "./RepositoryActivity";
import ActivityTrends from "./ActivityTrends";
import AnalyticsLayout from "./AnalyticsLayout";
import { Filters } from "../../services/api";

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

const AnalyticsDashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters] = useState<Filters>({
    timeRange: "7d",
    repo: "all",
  });

  useEffect(() => {
    // Initial data fetch
    const fetchData = async () => {
      try {
        // Fetch initial data here
        setLoading(false);
      } catch (error) {
        console.error("Error loading analytics data:", error);
        setError("Failed to load analytics data");
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return (
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        minHeight="100vh"
      >
        <CircularProgress />
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

  return (
    <AnalyticsLayout>
      <Box sx={{ flexGrow: 1, p: 3 }}>
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
