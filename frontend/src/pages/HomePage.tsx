import React from 'react';
import { Box, Heading, Button } from '@primer/react';
import { useNavigate } from 'react-router-dom';

const HomePage: React.FC = () => {
  const navigate = useNavigate();
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
      }}
    >
      <Heading sx={{ mb: 3 }}>Welcome to GitHub Dashboard</Heading>
      <Button onClick={() => navigate('/login')} sx={{ px: 4, py: 2 }}>
        Login
      </Button>
    </Box>
  );
};

export default HomePage;
