import React, { useState, useEffect } from "react";
import { Box, Grid, Paper, Typography, CircularProgress } from "@mui/material";
import { styled } from "@mui/material/styles";
import RepositoryActivity from "./RepositoryActivity";
import Trends from "./Trends";
import AnalyticsLayout from "./AnalyticsLayout";

const DashboardPaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(3),
  marginBottom: theme.spacing(3),
  height: "100%",
  display: "flex",
  flexDirection: "column",
}));

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

const AnalyticsDashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<Filters>({
    timeRange: "30",
    owner: "SHA888",
    repo: "github-dashboard",
  });

  const handleFilterChange = (newFilters: Filters) => {
    setFilters(newFilters);
    console.log("Filters changed:", newFilters);
  };

  useEffect(() => {
    // Initial data fetch
    const fetchData = async () => {
      try {
        // Fetch initial data here
        setLoading(false);
      } catch (err) {
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
    <Box sx={{ flexGrow: 1 }}>
      <AnalyticsLayout onFilterChange={handleFilterChange} />
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <DashboardPaper>
            <RepositoryActivity filters={filters} />
          </DashboardPaper>
        </Grid>
        <Grid item xs={12}>
          <DashboardPaper>
            <Trends filters={filters} />
          </DashboardPaper>
        </Grid>
      </Grid>
    </Box>
  );
};

export default AnalyticsDashboard;
