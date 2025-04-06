import React, { useState, useEffect } from "react";
import { Box, Typography, LinearProgress } from "@mui/material";
import RepositoryActivity from "./RepositoryActivity";
import Trends from "./Trends";
import AnalyticsLayout from "./AnalyticsLayout";

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
      } catch {
        setError('Failed to fetch analytics data');
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

  return (
    <Box sx={{ flexGrow: 1 }}>
      <Box sx={{ mb: 3 }}>
        <AnalyticsLayout onFilterChange={handleFilterChange} />
      </Box>
      <Box sx={{ mb: 3 }}>
        <RepositoryActivity filters={filters} />
      </Box>
      <Box>
        <Trends filters={filters} />
      </Box>
    </Box>
  );
};

export default AnalyticsDashboard;
