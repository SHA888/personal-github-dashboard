import React from 'react';
import { Box, Heading, Button, Text } from '@primer/react';
import { useAuth } from '../hooks/useAuth';

const DashboardPage: React.FC = () => {
  const { user, logout } = useAuth();
  return (
    <Box sx={{ p: 4 }}>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          mb: 4,
        }}
      >
        <Heading>Dashboard</Heading>
        {user && (
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Text sx={{ mr: 2 }}>{user.login}</Text>
            <Button variant="default" onClick={logout}>
              Logout
            </Button>
          </Box>
        )}
      </Box>
      <Text>Welcome to your GitHub Dashboard!</Text>
    </Box>
  );
};

export default DashboardPage;
