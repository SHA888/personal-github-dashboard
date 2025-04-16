import React from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  CircularProgress,
} from '@mui/material';
import { Code as CodeIcon } from '@mui/icons-material';
import { useEffect, useState } from 'react';
import { apiService } from '../services/api';

interface Repository {
  id: number;
  name: string;
  full_name: string;
  description: string | null;
  html_url: string;
  stargazers_count: number;
  forks_count: number;
  open_issues_count: number;
  language: string | null;
  updated_at: string;
  owner: {
    login: string;
    avatar_url: string;
  };
}

const Repositories: React.FC = () => {
  const [repositories, setRepositories] = useState<Repository[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchRepositories = async () => {
      try {
        setLoading(true);
        const response = await apiService.listRepositories();
        const repos = response.repositories.map(([owner, name]) => ({
          owner,
          name,
        }));
        setRepositories(repos);
        setError(null);
      } catch (err: Error) {
        console.error('Error fetching repositories:', err);
        setError(err.message || 'Failed to load repositories');
      } finally {
        setLoading(false);
      }
    };

    fetchRepositories();
  }, []);

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="200px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box p={3}>
      <Typography variant="h4" gutterBottom>
        Your Repositories
      </Typography>

      {error && (
        <Box mb={3}>
          <Typography color="error" variant="body2">
            {error}
          </Typography>
        </Box>
      )}

      <Card>
        <CardContent>
          {repositories.length === 0 ? (
            <Box p={3} textAlign="center">
              <Typography color="textSecondary">
                No repositories found. Add some repositories to get started.
              </Typography>
            </Box>
          ) : (
            <List>
              {repositories.map((repo) => (
                <ListItem
                  key={`${repo.owner}/${repo.name}`}
                  button
                  component="a"
                  href={`https://github.com/${repo.owner}/${repo.name}`}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <ListItemIcon>
                    <CodeIcon />
                  </ListItemIcon>
                  <ListItemText primary={repo.name} secondary={`Owner: ${repo.owner}`} />
                </ListItem>
              ))}
            </List>
          )}
        </CardContent>
      </Card>
    </Box>
  );
};

export default Repositories;
