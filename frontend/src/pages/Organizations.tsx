import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { apiService } from '../services/api';
import { Organization } from '../types/github';
import {
  Box,
  Typography,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
  Paper,
  Button,
} from '@mui/material';

const Organizations: React.FC = () => {
  const {
    data: organizations,
    isLoading,
    error,
    refetch,
  } = useQuery<Organization[]>('organizations', apiService.getOrganizations);

  const handleSync = async () => {
    try {
      await apiService.syncOrganizations();
      refetch(); // Refetch organizations after sync
    } catch (err) {
      console.error('Failed to sync organizations:', err);
      // TODO: Show error notification to the user
    }
  };

  if (isLoading) {
    return <CircularProgress />;
  }

  if (error) {
    return <Typography color="error">Failed to load organizations.</Typography>;
  }

  return (
    <Paper elevation={3} sx={{ padding: 3, marginTop: 3 }}>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
        <Typography variant="h5">Organizations</Typography>
        <Button variant="contained" onClick={handleSync}>
          Sync with GitHub
        </Button>
      </Box>
      <List>
        {organizations && organizations.length > 0 ? (
          organizations.map((org) => (
            <ListItem key={org.id}>
              <ListItemText primary={org.login} secondary={org.description || 'No description'} />
            </ListItem>
          ))
        ) : (
          <Typography>No organizations found.</Typography>
        )}
      </List>
    </Paper>
  );
};

export default Organizations;
