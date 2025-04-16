import React from 'react';
import { Box, Typography, Grid } from '@mui/material';
import ActivityTrends from './ActivityTrends';
import RepositoryActivity from './RepositoryActivity';

// Placeholder data - replace with actual API data
const activityTrendsData = {
  labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
  datasets: [
    {
      label: 'Commits',
      data: [65, 59, 80, 81, 56, 55],
      borderColor: 'rgb(75, 192, 192)',
      tension: 0.1,
    },
    {
      label: 'Issues Opened',
      data: [28, 48, 40, 19, 86, 27],
      borderColor: 'rgb(255, 99, 132)',
      tension: 0.1,
    },
  ],
};

const repositoryActivityData = [
  { name: 'Repo A', commits: 120, issues: 30, prs: 15 },
  { name: 'Repo B', commits: 85, issues: 12, prs: 8 },
  { name: 'Repo C', commits: 200, issues: 55, prs: 25 },
];

const AnalyticsDashboard: React.FC = () => {
  return (
    <Box sx={{ flexGrow: 1, padding: 3 }}>
      <Typography variant="h4" gutterBottom>
        Analytics Dashboard
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <ActivityTrends data={activityTrendsData} />
        </Grid>
        <Grid item xs={12} md={6}>
          <RepositoryActivity data={repositoryActivityData} />
        </Grid>
        {/* Add more analytics components here */}
      </Grid>
    </Box>
  );
};

export default AnalyticsDashboard;
