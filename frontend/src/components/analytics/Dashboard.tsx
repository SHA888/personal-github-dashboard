import React, { useState, useEffect } from "react";
import { Box, Typography, LinearProgress, Grid } from "@mui/material";
import RepositoryActivity from "./RepositoryActivity";
import Trends from "./Trends";
import AnalyticsLayout from "./AnalyticsLayout";
import { apiService } from "../../services/api";

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface RepositoryStats {
  id: number;
  name: string;
  owner: string;
  url: string;
  last_updated: string;
  stats: {
    stars: number;
    forks: number;
    open_issues: number;
    watchers: number;
    commit_count: number;
    contributors: number;
  };
}

const AnalyticsDashboard: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [repositoryStats, setRepositoryStats] = useState<RepositoryStats | null>(null);
  const [filters, setFilters] = useState<Filters>({
    timeRange: "30",
    owner: "SHA888",
    repo: "github-dashboard",
  });

  const handleFilterChange = (newFilters: Filters) => {
    setFilters(newFilters);
  };

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const [repoResponse] = await Promise.all([
          apiService.getRepositoryDetails(filters.owner, filters.repo),
        ]);
        setRepositoryStats(repoResponse.data);
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

      {repositoryStats && (
        <Box sx={{ mb: 3 }}>
          <Grid container spacing={2}>
            <Grid item xs={12} sm={6} md={3}>
              <Box sx={{ p: 2, bgcolor: "background.paper", borderRadius: 1 }}>
                <Typography variant="subtitle2" color="text.secondary">
                  Stars
                </Typography>
                <Typography variant="h4">{repositoryStats.stats.stars}</Typography>
              </Box>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Box sx={{ p: 2, bgcolor: "background.paper", borderRadius: 1 }}>
                <Typography variant="subtitle2" color="text.secondary">
                  Forks
                </Typography>
                <Typography variant="h4">{repositoryStats.stats.forks}</Typography>
              </Box>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Box sx={{ p: 2, bgcolor: "background.paper", borderRadius: 1 }}>
                <Typography variant="subtitle2" color="text.secondary">
                  Open Issues
                </Typography>
                <Typography variant="h4">{repositoryStats.stats.open_issues}</Typography>
              </Box>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Box sx={{ p: 2, bgcolor: "background.paper", borderRadius: 1 }}>
                <Typography variant="subtitle2" color="text.secondary">
                  Contributors
                </Typography>
                <Typography variant="h4">{repositoryStats.stats.contributors}</Typography>
              </Box>
            </Grid>
          </Grid>
        </Box>
      )}

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
