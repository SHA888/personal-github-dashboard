import React from 'react';
import { useSelector } from 'react-redux';
import {
  Box,
  Typography,
  Paper,
  List,
  ListItem,
  ListItemText,
  CircularProgress,
} from '@mui/material';
import { RootState } from '../../store';
import { RepositoryActivityData } from '../../store/slices/analyticsSlice'; // Assuming type defined here

interface RepositoryActivityProps {
  // data passed directly now, filters removed
  data: RepositoryActivityData[];
}

const RepositoryActivity: React.FC<RepositoryActivityProps> = ({ data }) => {
  const { loading, error } = useSelector((state: RootState) => state.analytics);

  if (loading) {
    return <CircularProgress />;
  }

  if (error) {
    return <Typography color="error">Error loading repository activity: {error}</Typography>;
  }

  return (
    <Paper elevation={3} sx={{ padding: 2 }}>
      <Typography variant="h6" gutterBottom>
        Repository Activity Overview
      </Typography>
      <List>
        {data.map((repo) => (
          <ListItem key={repo.name}>
            <ListItemText
              primary={repo.name}
              secondary={`Commits: ${repo.commits}, Issues: ${repo.issues}, PRs: ${repo.prs}`}
            />
          </ListItem>
        ))}
      </List>
    </Paper>
  );
};

export default RepositoryActivity;
