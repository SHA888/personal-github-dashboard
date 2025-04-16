import React, { useEffect, useState } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { Box, CircularProgress, Typography } from '@mui/material';
import { fetchRepositoryActivity } from '../../store/slices/analyticsSlice';
import { RootState, AppDispatch } from '../../store';
import ActivityTrends from './ActivityTrends';
import RepositoryActivity from './RepositoryActivity';

const Dashboard: React.FC = () => {
  const dispatch = useDispatch<AppDispatch>();
  const { loading, error } = useSelector((state: RootState) => state.analytics);
  // Placeholder data until API is fully integrated
  const [activityTrendsData, setActivityTrendsData] = useState<any>(null);
  const [repositoryActivityData, setRepositoryActivityData] = useState<any>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        // Fetch repository activity data
        // const activityResponse = await dispatch(fetchRepositoryActivity()).unwrap(); // Commented out unused variable
        await dispatch(fetchRepositoryActivity()).unwrap();

        // TODO: Fetch activity trends data
        // For now, use placeholder data
        setActivityTrendsData({
          labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
          datasets: [
            {
              label: 'Commits',
              data: [65, 59, 80, 81, 56, 55],
              borderColor: 'rgb(75, 192, 192)',
              tension: 0.1,
            },
          ],
        });

        // TODO: Process activityResponse into repositoryActivityData format
        // const activityData = processActivityData(activityResponse); // Commented out unused variable
        // For now, use placeholder data
        setRepositoryActivityData([
          { name: 'Repo A', commits: 120, issues: 30, prs: 15 },
          { name: 'Repo B', commits: 85, issues: 12, prs: 8 },
        ]);
      } catch (err) {
        console.error('Failed to fetch analytics data:', err);
        // Error state is handled by the slice
      }
    };

    fetchData();
  }, [dispatch]);

  if (loading) {
    return <CircularProgress />;
  }

  if (error) {
    return <Typography color="error">{error}</Typography>;
  }

  return (
    <Box>
      {activityTrendsData && <ActivityTrends data={activityTrendsData} />}
      {repositoryActivityData && <RepositoryActivity data={repositoryActivityData} />}
    </Box>
  );
};

export default Dashboard;
