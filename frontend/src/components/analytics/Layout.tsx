import React, { useState, useEffect } from 'react';
import { Box, TextField, MenuItem, Grid, Autocomplete } from '@mui/material';
import RepositoryActivity from './RepositoryActivity';
import ActivityTrends from './ActivityTrends';
import { apiService, Repository } from '../../services/api';

interface Filters {
  timeRange: string;
  owner: string;
  repo: string;
}

interface AnalyticsLayoutProps {
  onFilterChange: (filters: Filters) => void;
}

const AnalyticsLayout: React.FC<AnalyticsLayoutProps> = ({ onFilterChange }) => {
  const [filters, setFilters] = useState<Filters>({
    timeRange: '30',
    owner: 'SHA888',
    repo: 'github-dashboard',
  });
  const [repositories, setRepositories] = useState<Repository[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchRepositories = async () => {
      try {
        setLoading(true);
        const response = await apiService.listRepositories();
        setRepositories(response.data.repositories);
      } catch (error) {
        console.error('Error fetching repositories:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchRepositories();
  }, []);

  const handleFilterChange = (field: keyof Filters, value: string) => {
    const newFilters = { ...filters, [field]: value };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  return (
    <Box sx={{ flexGrow: 1, p: 3 }}>
      {/* Filter Controls */}
      <Box sx={{ mb: 3 }}>
        <Grid container spacing={2}>
          <Grid item xs={12} sm={6} md={3}>
            <TextField
              select
              fullWidth
              label="Time Range"
              value={filters.timeRange}
              onChange={(e) => handleFilterChange('timeRange', e.target.value)}
            >
              <MenuItem value="7">Last 7 days</MenuItem>
              <MenuItem value="30">Last 30 days</MenuItem>
              <MenuItem value="90">Last 90 days</MenuItem>
              <MenuItem value="180">Last 180 days</MenuItem>
              <MenuItem value="365">Last year</MenuItem>
            </TextField>
          </Grid>
          <Grid item xs={12} sm={6} md={3}>
            <Autocomplete
              options={repositories}
              getOptionLabel={(option) => option.owner}
              value={repositories.find((repo) => repo.owner === filters.owner) || null}
              onChange={(_, newValue) => {
                if (newValue) {
                  handleFilterChange('owner', newValue.owner);
                }
              }}
              renderInput={(params) => <TextField {...params} label="Owner" fullWidth />}
              loading={loading}
            />
          </Grid>
          <Grid item xs={12} sm={6} md={3}>
            <Autocomplete
              options={repositories.filter((repo) => repo.owner === filters.owner)}
              getOptionLabel={(option) => option.name}
              value={repositories.find((repo) => repo.name === filters.repo) || null}
              onChange={(_, newValue) => {
                if (newValue) {
                  handleFilterChange('repo', newValue.name);
                }
              }}
              renderInput={(params) => <TextField {...params} label="Repository" fullWidth />}
              loading={loading}
            />
          </Grid>
        </Grid>
      </Box>

      {/* Main Content */}
      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 3 }}>
        <Box sx={{ flex: '1 1 400px', minWidth: 0 }}>
          <RepositoryActivity filters={filters} />
        </Box>
        <Box sx={{ flex: '1 1 400px', minWidth: 0 }}>
          <ActivityTrends filters={filters} />
        </Box>
      </Box>
    </Box>
  );
};

export default AnalyticsLayout;
